export type Locale = "en" | "zh";

const STORAGE_KEY = "agent-desk-locale";

const messages = {
  en: {
    "app.eyebrow": "Agent Desk",
    "app.title": "Local agent health",
    "app.subtitle": "Desktop agent diagnostics",
    "app.footer": "Local runtime discovery · config · health checks",
    "health.ready": "Ready",
    "health.scanning": "Scanning…",
    "health.good": "All runtimes healthy",
    "health.partial": "{installed}/{total} runtimes installed",
    "health.bad": "No runtimes installed",
    "summary.installed": "Installed",
    "summary.preset": "Preset",
    "summary.lastScan": "Last scan",
    "presets.title": "Presets",
    "presets.loading": "Loading presets…",
    "presets.switch": "Switch",
    "presets.none": "No presets yet",
    "presets.noneHint": "Run: agent-desk profile init",
    "presets.active": "Active preset: {name}",
    "presets.noActive": "No active preset selected",
    "presets.switchHint": "Applies Hermes config and creates a backup. Local uses Ollama with no API key.",
    "presets.localMeta": "ollama · {model} · no API key",
    "presets.failed": "Failed to load presets",
    "presets.applying": "Applying {name}…",
    "presets.updated": "Updated: {list}. Restart affected runtimes if needed.",
    "doctor.title": "Doctor",
    "doctor.loading": "Loading…",
    "doctor.run": "Scan",
    "doctor.running": "Running doctor…",
    "doctor.companyOk": "Company profile detected. Runtimes scanned successfully.",
    "doctor.companyMissing": "No company profile yet. Local discovery still works.",
    "doctor.failed": "Doctor failed: {error}",
    "doctor.empty": "Could not complete the scan. Try again.",
    "runtimes.title": "Runtimes",
    "runtimes.tracked": "{count} tracked",
    "runtimes.empty": "No runtime adapters configured.",
    "runtime.installed": "installed",
    "runtime.notInstalled": "not installed",
    "runtime.edit": "Edit",
    "runtime.cancel": "Cancel",
    "runtime.save": "Save",
    "runtime.saveHint": "Updates ~/.hermes/config.yaml and .env with automatic backup.",
    "runtime.saving": "Saving…",
    "meta.provider": "Provider",
    "meta.model": "Model",
    "meta.gateway": "Gateway",
    "meta.version": "Version",
    "meta.binary": "Binary",
    "meta.config": "Config",
    "meta.apiKey": "API Key",
    "meta.secrets": "Secrets",
    "meta.apiKeySet": "Configured ({hint})",
    "meta.apiKeyMissing": "Not set ({env})",
    "meta.apiKeyOptional": "Not required for this provider",
    "meta.apiKeyPlaceholder": "Leave blank to keep current key",
    "status.none": "None",
    "status.error": "Error",
    "lang.en": "EN",
    "lang.zh": "中文",
  },
  zh: {
    "app.eyebrow": "Agent Desk",
    "app.title": "本机 Agent 状态",
    "app.subtitle": "桌面 Agent 诊断工具",
    "app.footer": "本机 Runtime 发现 · 配置 · 健康检查",
    "health.ready": "就绪",
    "health.scanning": "扫描中…",
    "health.good": "Runtime 状态正常",
    "health.partial": "已安装 {installed}/{total}",
    "health.bad": "暂无已安装 Runtime",
    "summary.installed": "已安装",
    "summary.preset": "预设",
    "summary.lastScan": "上次扫描",
    "presets.title": "配置预设",
    "presets.loading": "正在加载预设…",
    "presets.switch": "切换",
    "presets.none": "暂无预设",
    "presets.noneHint": "运行：agent-desk profile init",
    "presets.active": "当前预设：{name}",
    "presets.noActive": "未选择预设",
    "presets.switchHint": "会更新 Hermes 配置并自动备份。local 走 Ollama，无需 API Key。",
    "presets.localMeta": "ollama · {model} · 无需 API Key",
    "presets.failed": "加载预设失败",
    "presets.applying": "正在应用 {name}…",
    "presets.updated": "已更新：{list}。请重启相关 Runtime。",
    "doctor.title": "诊断",
    "doctor.loading": "加载中…",
    "doctor.run": "扫描",
    "doctor.running": "正在诊断…",
    "doctor.companyOk": "已检测到企业配置，Runtime 扫描完成。",
    "doctor.companyMissing": "尚无企业配置，本机发现功能仍可用。",
    "doctor.failed": "诊断失败：{error}",
    "doctor.empty": "无法完成扫描，请重试。",
    "runtimes.title": "Runtime",
    "runtimes.tracked": "共 {count} 项",
    "runtimes.empty": "没有可扫描的 Runtime。",
    "runtime.installed": "已安装",
    "runtime.notInstalled": "未安装",
    "runtime.edit": "编辑",
    "runtime.cancel": "取消",
    "runtime.save": "保存",
    "runtime.saveHint": "更新 ~/.hermes/config.yaml 与 .env，并自动备份。",
    "runtime.saving": "保存中…",
    "meta.provider": "提供商",
    "meta.model": "模型",
    "meta.gateway": "网关",
    "meta.version": "版本",
    "meta.binary": "可执行文件",
    "meta.config": "配置文件",
    "meta.apiKey": "API Key",
    "meta.secrets": "密钥文件",
    "meta.apiKeySet": "已配置（{hint}）",
    "meta.apiKeyMissing": "未配置（{env}）",
    "meta.apiKeyOptional": "此提供商无需 API Key",
    "meta.apiKeyPlaceholder": "留空则保留现有 Key",
    "status.none": "无",
    "status.error": "错误",
    "lang.en": "EN",
    "lang.zh": "中文",
  },
} as const;

export type MessageKey = keyof (typeof messages)["en"];

let locale: Locale = detectLocale();

function detectLocale(): Locale {
  const saved = localStorage.getItem(STORAGE_KEY);
  if (saved === "en" || saved === "zh") {
    return saved;
  }
  const lang = navigator.language.toLowerCase();
  return lang.startsWith("zh") ? "zh" : "en";
}

export function getLocale(): Locale {
  return locale;
}

export function setLocale(next: Locale): void {
  locale = next;
  localStorage.setItem(STORAGE_KEY, next);
  document.documentElement.lang = next === "zh" ? "zh-CN" : "en";
}

export function t(key: MessageKey, params?: Record<string, string>): string {
  let text: string = messages[locale][key] ?? messages.en[key] ?? key;
  if (params) {
    for (const [name, value] of Object.entries(params)) {
      text = text.replace(`{${name}}`, value);
    }
  }
  return text;
}

export function applyStaticI18n(root: ParentNode = document): void {
  root.querySelectorAll<HTMLElement>("[data-i18n]").forEach((element) => {
    const key = element.dataset.i18n as MessageKey | undefined;
    if (!key) {
      return;
    }
    element.textContent = t(key);
  });

  const presetTrigger = document.querySelector<HTMLButtonElement>("#preset-trigger");
  if (presetTrigger) {
    presetTrigger.setAttribute(
      "aria-label",
      locale === "zh" ? "配置预设" : "Profile preset",
    );
  }
}
