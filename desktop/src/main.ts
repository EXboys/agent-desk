import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  applyStaticI18n,
  getLocale,
  setLocale,
  t,
  type Locale,
} from "./i18n";

interface RuntimeDoctorResult {
  id: string;
  display_name: string;
  installed: boolean;
  version: string | null;
  binary_path: string | null;
  config_paths: string[];
  profile: {
    gateway_url: string | null;
    key_source: string | null;
  };
}

interface DoctorReport {
  profile_env_path: string | null;
  profile_env_exists: boolean;
  active_preset: string | null;
  runtimes: RuntimeDoctorResult[];
}

interface HermesSettings {
  provider: string;
  model: string;
  base_url: string;
  api_key_env: string | null;
  api_key_configured: boolean;
  api_key_hint: string | null;
}

interface ProfileEntry {
  hermes?: Pick<HermesSettings, "provider" | "model" | "base_url">;
}

interface ProfilesDocument {
  active: string | null;
  profiles: Record<string, ProfileEntry>;
}

interface UseProfileReport {
  profile: string;
  applied: Array<{
    runtime_id: string;
    config_path: string;
    backup_path: string | null;
    restart_hint: string;
  }>;
  skipped: string[];
}

const statusEl = document.querySelector<HTMLElement>("#status")!;
const runtimesEl = document.querySelector<HTMLElement>("#runtimes")!;
const refreshBtn = document.querySelector<HTMLButtonElement>("#refresh")!;
const spinnerEl = refreshBtn.querySelector<HTMLElement>(".spinner")!;
const installedCountEl = document.querySelector<HTMLElement>("#installed-count")!;
const profileStatusEl = document.querySelector<HTMLElement>("#profile-status")!;
const lastScanEl = document.querySelector<HTMLElement>("#last-scan")!;
const runtimeCountEl = document.querySelector<HTMLElement>("#runtime-count")!;
const presetStatusEl = document.querySelector<HTMLElement>("#preset-status")!;
const presetSelectEl = document.querySelector<HTMLSelectElement>("#preset-select")!;
const presetApplyEl = document.querySelector<HTMLButtonElement>("#preset-apply")!;
const presetHintEl = document.querySelector<HTMLElement>("#preset-hint")!;
const langSwitchEl = document.querySelector<HTMLElement>(".lang-switch")!;

const RUNTIME_SHORT: Record<string, string> = {
  openclaw: "OC",
  hermes: "HE",
  "claude-code": "CC",
};

let lastReport: DoctorReport | null = null;
let lastProfiles: ProfilesDocument | null = null;
let hermesModel: HermesSettings | null = null;
let hermesEditing = false;

function escapeHtml(value: string): string {
  return value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function formatTime(date: Date): string {
  const locale = getLocale() === "zh" ? "zh-CN" : "en-US";
  return date.toLocaleTimeString(locale, {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
}

function runtimeClass(id: string): string {
  if (id in RUNTIME_SHORT) {
    return id;
  }
  return "default";
}

function runtimeInitials(id: string, displayName: string): string {
  return RUNTIME_SHORT[id] ?? displayName.slice(0, 2).toUpperCase();
}

function metaRow(labelKey: Parameters<typeof t>[0], value: string): string {
  return `
    <div class="meta-row">
      <span class="meta-label">${t(labelKey)}</span>
      <p class="meta-value">${escapeHtml(value)}</p>
    </div>
  `;
}

function metaInput(
  labelKey: Parameters<typeof t>[0],
  field: string,
  value: string,
  inputType = "text",
  placeholder = "",
): string {
  return `
    <label class="meta-row meta-row-edit">
      <span class="meta-label">${t(labelKey)}</span>
      <input class="meta-input" data-field="${field}" type="${inputType}" value="${escapeHtml(value)}" placeholder="${escapeHtml(placeholder)}" />
    </label>
  `;
}

function renderApiKeyRow(settings: HermesSettings): string {
  if (!settings.api_key_env) {
    return metaRow("meta.apiKey", t("meta.apiKeyOptional"));
  }
  if (settings.api_key_configured && settings.api_key_hint) {
    return metaRow(
      "meta.apiKey",
      t("meta.apiKeySet", { hint: settings.api_key_hint }),
    );
  }
  return metaRow(
    "meta.apiKey",
    t("meta.apiKeyMissing", { env: settings.api_key_env }),
  );
}

function renderHermesCard(runtime: RuntimeDoctorResult): string {
  const model = hermesModel ?? {
    provider: "",
    model: "",
    base_url: runtime.profile.gateway_url ?? "",
    api_key_env: null,
    api_key_configured: false,
    api_key_hint: null,
  };

  const editButton = hermesEditing
    ? ""
    : `<button type="button" class="btn-ghost" data-action="edit-hermes">${t("runtime.edit")}</button>`;

  const meta = hermesEditing
    ? [
        metaInput("meta.provider", "provider", model.provider),
        metaInput("meta.model", "model", model.model),
        metaInput("meta.gateway", "base_url", model.base_url),
        model.api_key_env
          ? metaInput(
              "meta.apiKey",
              "api_key",
              "",
              "password",
              t("meta.apiKeyPlaceholder"),
            )
          : "",
      ].join("")
    : [
        model.provider ? metaRow("meta.provider", model.provider) : "",
        model.model ? metaRow("meta.model", model.model) : "",
        model.base_url ? metaRow("meta.gateway", model.base_url) : "",
        renderApiKeyRow(model),
        runtime.profile.key_source
          ? metaRow("meta.secrets", runtime.profile.key_source)
          : "",
        runtime.version ? metaRow("meta.version", runtime.version) : "",
        runtime.binary_path ? metaRow("meta.binary", runtime.binary_path) : "",
        runtime.config_paths.length
          ? metaRow("meta.config", runtime.config_paths.join("\n"))
          : "",
      ]
        .filter(Boolean)
        .join("");

  const actions = hermesEditing
    ? `
      <div class="card-actions">
        <button type="button" class="btn-secondary" data-action="cancel-hermes">${t("runtime.cancel")}</button>
        <button type="button" class="btn-primary" data-action="save-hermes">${t("runtime.save")}</button>
      </div>
      <p class="card-hint" data-hermes-hint>${t("runtime.saveHint")}</p>
    `
    : "";

  return `
    <article class="runtime hermes ${hermesEditing ? "is-editing" : ""}" data-runtime="hermes">
      <div class="runtime-head">
        <div class="runtime-title">
          <div class="runtime-icon">${runtimeInitials(runtime.id, runtime.display_name)}</div>
          <div>
            <h3>${runtime.display_name}</h3>
            <p class="runtime-id">${runtime.id}</p>
          </div>
        </div>
        <div class="runtime-actions">
          ${editButton}
          <p class="badge ok">${t("runtime.installed")}</p>
        </div>
      </div>
      ${meta ? `<div class="meta-grid">${meta}</div>` : ""}
      ${actions}
    </article>
  `;
}

function renderRuntimeCard(runtime: RuntimeDoctorResult): string {
  if (runtime.id === "hermes" && runtime.installed) {
    return renderHermesCard(runtime);
  }

  const state = runtime.installed ? t("runtime.installed") : t("runtime.notInstalled");
  const badgeClass = runtime.installed ? "ok" : "muted";
  const rows = [
    runtime.version ? metaRow("meta.version", runtime.version) : "",
    runtime.binary_path ? metaRow("meta.binary", runtime.binary_path) : "",
    runtime.config_paths.length ? metaRow("meta.config", runtime.config_paths.join("\n")) : "",
    runtime.profile.gateway_url ? metaRow("meta.gateway", runtime.profile.gateway_url) : "",
  ]
    .filter(Boolean)
    .join("");

  return `
    <article class="runtime ${runtimeClass(runtime.id)}" data-runtime="${runtime.id}">
      <div class="runtime-head">
        <div class="runtime-title">
          <div class="runtime-icon">${runtimeInitials(runtime.id, runtime.display_name)}</div>
          <div>
            <h3>${runtime.display_name}</h3>
            <p class="runtime-id">${runtime.id}</p>
          </div>
        </div>
        <p class="badge ${badgeClass}">${state}</p>
      </div>
      ${rows ? `<div class="meta-grid">${rows}</div>` : ""}
    </article>
  `;
}

async function loadHermesModel(): Promise<void> {
  try {
    hermesModel = await invoke<HermesSettings>("get_hermes_model_command");
  } catch {
    hermesModel = null;
  }
}

async function renderReport(report: DoctorReport) {
  lastReport = report;
  const installed = report.runtimes.filter((runtime) => runtime.installed).length;
  const total = report.runtimes.length;

  installedCountEl.textContent = `${installed}/${total}`;
  profileStatusEl.textContent = report.active_preset ?? t("status.none");
  lastScanEl.textContent = formatTime(new Date());
  runtimeCountEl.textContent = t("runtimes.tracked", { count: String(total) });

  statusEl.textContent = report.profile_env_exists
    ? t("doctor.companyOk")
    : t("doctor.companyMissing");

  if (report.runtimes.some((runtime) => runtime.id === "hermes" && runtime.installed)) {
    await loadHermesModel();
  } else {
    hermesModel = null;
    hermesEditing = false;
  }

  if (report.runtimes.length === 0) {
    runtimesEl.innerHTML = `<div class="empty-state">${t("runtimes.empty")}</div>`;
    return;
  }

  runtimesEl.innerHTML = report.runtimes.map(renderRuntimeCard).join("");
}

function renderProfiles(doc: ProfilesDocument) {
  lastProfiles = doc;
  const names = Object.keys(doc.profiles);
  presetSelectEl.innerHTML = "";

  if (names.length === 0) {
    presetStatusEl.textContent = t("presets.none");
    presetApplyEl.disabled = true;
    presetHintEl.textContent = t("presets.noneHint");
    return;
  }

  presetStatusEl.textContent = doc.active
    ? t("presets.active", { name: doc.active })
    : t("presets.noActive");

  for (const name of names) {
    const option = document.createElement("option");
    option.value = name;
    option.textContent = name;
    option.selected = doc.active === name;
    presetSelectEl.appendChild(option);
  }

  presetApplyEl.disabled = false;
  presetHintEl.textContent = t("presets.switchHint");
}

async function loadProfiles() {
  try {
    const doc = await invoke<ProfilesDocument>("list_profiles_command");
    renderProfiles(doc);
  } catch (error) {
    presetStatusEl.textContent = t("presets.failed");
    presetHintEl.textContent = String(error);
    presetApplyEl.disabled = true;
  }
}

function setLoading(loading: boolean) {
  refreshBtn.disabled = loading;
  refreshBtn.classList.toggle("is-loading", loading);
  spinnerEl.hidden = !loading;
  runtimesEl.classList.toggle("is-loading", loading);
}

async function refresh() {
  setLoading(true);
  statusEl.textContent = t("doctor.running");
  try {
    const report = await invoke<DoctorReport>("run_doctor_command");
    await renderReport(report);
  } catch (error) {
    statusEl.textContent = t("doctor.failed", { error: String(error) });
    runtimesEl.innerHTML = `<div class="empty-state">${t("doctor.empty")}</div>`;
    installedCountEl.textContent = "—";
    profileStatusEl.textContent = t("status.error");
    runtimeCountEl.textContent = "—";
  } finally {
    setLoading(false);
  }
}

async function applyPreset() {
  const name = presetSelectEl.value;
  if (!name) {
    return;
  }

  presetApplyEl.disabled = true;
  presetHintEl.textContent = t("presets.applying", { name });
  try {
    const report = await invoke<UseProfileReport>("use_profile_command", { name });
    const applied = report.applied.map((item) => item.runtime_id).join(", ");
    presetHintEl.textContent = applied
      ? t("presets.updated", { list: applied })
      : report.skipped.join("; ");
    hermesEditing = false;
    await loadProfiles();
    await refresh();
  } catch (error) {
    presetHintEl.textContent = String(error);
  } finally {
    presetApplyEl.disabled = false;
  }
}

function readHermesDraft(card: HTMLElement): {
  provider: string;
  model: string;
  base_url: string;
  api_key: string;
} {
  const read = (field: string) =>
    card.querySelector<HTMLInputElement>(`[data-field="${field}"]`)?.value.trim() ?? "";
  return {
    provider: read("provider"),
    model: read("model"),
    base_url: read("base_url"),
    api_key: read("api_key"),
  };
}

async function saveHermesCard(card: HTMLElement) {
  const hint = card.querySelector<HTMLElement>("[data-hermes-hint]");
  const saveBtn = card.querySelector<HTMLButtonElement>('[data-action="save-hermes"]');
  const draft = readHermesDraft(card);

  saveBtn?.setAttribute("disabled", "true");
  if (hint) {
    hint.textContent = t("runtime.saving");
  }

  try {
    const payload = {
      provider: draft.provider,
      model: draft.model,
      base_url: draft.base_url,
      api_key: draft.api_key ? draft.api_key : null,
    };
    const report = await invoke<{
      restart_hint: string;
      backup_path: string | null;
    }>("set_hermes_model_command", payload);
    hermesEditing = false;
    if (hint) {
      hint.textContent = report.restart_hint;
    }
    await refresh();
  } catch (error) {
    if (hint) {
      hint.textContent = String(error);
    }
  } finally {
    saveBtn?.removeAttribute("disabled");
  }
}

function updateLangButtons() {
  const current = getLocale();
  langSwitchEl.querySelectorAll<HTMLButtonElement>(".lang-btn").forEach((button) => {
    const active = button.dataset.lang === current;
    button.classList.toggle("is-active", active);
    button.setAttribute("aria-pressed", String(active));
  });
}

async function switchLocale(next: Locale) {
  if (next === getLocale()) {
    return;
  }
  setLocale(next);
  applyStaticI18n();
  updateLangButtons();
  if (lastProfiles) {
    renderProfiles(lastProfiles);
  }
  if (lastReport) {
    await renderReport(lastReport);
  } else {
    statusEl.textContent = t("doctor.loading");
    presetStatusEl.textContent = t("presets.loading");
  }
}

runtimesEl.addEventListener("click", (event) => {
  const target = event.target as HTMLElement;
  const action = target.closest<HTMLElement>("[data-action]")?.dataset.action;
  if (!action) {
    return;
  }

  const card = target.closest<HTMLElement>('[data-runtime="hermes"]');
  if (!card) {
    return;
  }

  if (action === "edit-hermes") {
    hermesEditing = true;
    if (lastReport) {
      void renderReport(lastReport);
    }
    return;
  }

  if (action === "cancel-hermes") {
    hermesEditing = false;
    if (lastReport) {
      void renderReport(lastReport);
    }
    return;
  }

  if (action === "save-hermes") {
    void saveHermesCard(card);
  }
});

langSwitchEl.addEventListener("click", (event) => {
  const button = (event.target as HTMLElement).closest<HTMLButtonElement>(".lang-btn");
  const lang = button?.dataset.lang;
  if (lang === "en" || lang === "zh") {
    void switchLocale(lang);
  }
});

refreshBtn.addEventListener("click", () => {
  void refresh();
});

presetApplyEl.addEventListener("click", () => {
  void applyPreset();
});

void listen<DoctorReport>("doctor-report", (event) => {
  void renderReport(event.payload);
});

setLocale(getLocale());
applyStaticI18n();
updateLangButtons();
void loadProfiles();
void refresh();
