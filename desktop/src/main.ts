import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

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
  runtimes: RuntimeDoctorResult[];
}

const statusEl = document.querySelector<HTMLElement>("#status")!;
const runtimesEl = document.querySelector<HTMLElement>("#runtimes")!;
const refreshBtn = document.querySelector<HTMLButtonElement>("#refresh")!;
const spinnerEl = refreshBtn.querySelector<HTMLElement>(".spinner")!;
const installedCountEl = document.querySelector<HTMLElement>("#installed-count")!;
const profileStatusEl = document.querySelector<HTMLElement>("#profile-status")!;
const lastScanEl = document.querySelector<HTMLElement>("#last-scan")!;
const runtimeCountEl = document.querySelector<HTMLElement>("#runtime-count")!;

const RUNTIME_SHORT: Record<string, string> = {
  openclaw: "OC",
  hermes: "HE",
  "claude-code": "CC",
};

function formatTime(date: Date): string {
  return date.toLocaleTimeString([], {
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

function metaRow(label: string, value: string): string {
  return `
    <div class="meta-row">
      <span class="meta-label">${label}</span>
      <p class="meta-value">${value}</p>
    </div>
  `;
}

function renderReport(report: DoctorReport) {
  const installed = report.runtimes.filter((runtime) => runtime.installed).length;
  const total = report.runtimes.length;

  installedCountEl.textContent = `${installed}/${total}`;
  profileStatusEl.textContent = report.profile_env_exists ? "Ready" : "Missing";
  lastScanEl.textContent = formatTime(new Date());
  runtimeCountEl.textContent = `${total} tracked`;

  statusEl.textContent = report.profile_env_exists
    ? "Company profile detected. Runtimes scanned successfully."
    : "No company profile yet. Local discovery still works.";

  if (report.runtimes.length === 0) {
    runtimesEl.innerHTML =
      '<div class="empty-state">No runtime adapters configured.</div>';
    return;
  }

  runtimesEl.innerHTML = report.runtimes
    .map((runtime) => {
      const state = runtime.installed ? "installed" : "not installed";
      const badgeClass = runtime.installed ? "ok" : "muted";
      const rows = [
        runtime.version ? metaRow("Version", runtime.version) : "",
        runtime.binary_path ? metaRow("Binary", runtime.binary_path) : "",
        runtime.config_paths.length
          ? metaRow("Config", runtime.config_paths.join("\n"))
          : "",
        runtime.profile.gateway_url
          ? metaRow("Gateway", runtime.profile.gateway_url)
          : "",
      ]
        .filter(Boolean)
        .join("");

      return `
        <article class="runtime ${runtimeClass(runtime.id)}">
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
    })
    .join("");
}

function setLoading(loading: boolean) {
  refreshBtn.disabled = loading;
  refreshBtn.classList.toggle("is-loading", loading);
  spinnerEl.hidden = !loading;
}

async function refresh() {
  setLoading(true);
  statusEl.textContent = "Running doctor…";
  try {
    const report = await invoke<DoctorReport>("run_doctor_command");
    renderReport(report);
  } catch (error) {
    statusEl.textContent = `Doctor failed: ${String(error)}`;
    runtimesEl.innerHTML =
      '<div class="empty-state">Could not complete the scan. Try again.</div>';
    installedCountEl.textContent = "—";
    profileStatusEl.textContent = "Error";
    runtimeCountEl.textContent = "—";
  } finally {
    setLoading(false);
  }
}

refreshBtn.addEventListener("click", () => {
  void refresh();
});

void listen<DoctorReport>("doctor-report", (event) => {
  renderReport(event.payload);
});

void refresh();
