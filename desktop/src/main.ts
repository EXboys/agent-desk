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

function renderReport(report: DoctorReport) {
  const configured = report.profile_env_exists;
  statusEl.textContent = configured
    ? "Company profile found on this machine."
    : "No company profile yet — run setup when your team provides one.";

  runtimesEl.innerHTML = report.runtimes
    .map((runtime) => {
      const state = runtime.installed ? "installed" : "not installed";
      const gateway = runtime.profile.gateway_url
        ? `<p class="meta">gateway: ${runtime.profile.gateway_url}</p>`
        : "";
      const binary = runtime.binary_path
        ? `<p class="meta">binary: ${runtime.binary_path}</p>`
        : "";
      return `
        <article class="runtime">
          <h2>${runtime.display_name}</h2>
          <p class="badge ${runtime.installed ? "ok" : "muted"}">${state}</p>
          ${runtime.version ? `<p class="meta">version: ${runtime.version}</p>` : ""}
          ${binary}
          ${gateway}
        </article>
      `;
    })
    .join("");
}

async function refresh() {
  refreshBtn.disabled = true;
  statusEl.textContent = "Running doctor…";
  try {
    const report = await invoke<DoctorReport>("run_doctor_command");
    renderReport(report);
  } catch (error) {
    statusEl.textContent = `Doctor failed: ${String(error)}`;
    runtimesEl.innerHTML = "";
  } finally {
    refreshBtn.disabled = false;
  }
}

refreshBtn.addEventListener("click", () => {
  void refresh();
});

void listen<DoctorReport>("doctor-report", (event) => {
  renderReport(event.payload);
});

void refresh();
