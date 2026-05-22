const { invoke } = window.__TAURI__.core;

const apiKeyInput = document.getElementById("api-key");
const languageSelect = document.getElementById("language");
const statusPill = document.getElementById("status-pill");
const statusText = statusPill.querySelector(".status-text");
const saveButton = document.getElementById("save-config");
const testButton = document.getElementById("test-api-key");
const toast = document.getElementById("toast");

let toastTimer = null;

function setStatus(text, isError = false) {
  statusText.textContent = text;
  statusPill.classList.toggle("error", isError);
}

function showToast(message) {
  const toastText = toast.querySelector(".toast-text");
  toastText.textContent = message;
  toast.classList.add("visible");
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => toast.classList.remove("visible"), 2200);
}

function populateForm(config) {
  apiKeyInput.value = config.api_key ?? "";
  languageSelect.value = config.language ?? "auto";
}

async function loadConfig() {
  try {
    const config = await invoke("get_config");
    populateForm(config);
    setStatus("Ready");
  } catch (error) {
    setStatus(`Error: ${error}`, true);
  }
}

saveButton.addEventListener("click", async () => {
  const config = {
    api_key: apiKeyInput.value.trim(),
    language: languageSelect.value,
    hotkey: "Ctrl+Win",
  };

  try {
    await invoke("save_config_cmd", { config });
    setStatus("Ready");
    showToast("Settings saved");
  } catch (error) {
    setStatus("Save failed", true);
  }
});

testButton.addEventListener("click", async () => {
  const apiKey = apiKeyInput.value.trim();
  if (!apiKey) {
    setStatus("API key missing", true);
    return;
  }

  setStatus("Testing\u2026");
  try {
    await invoke("test_api_key", { apiKey });
    setStatus("API key valid");
    showToast("API key is valid");
  } catch (error) {
    setStatus("API key invalid", true);
  }
});

document.getElementById("get-api-key").addEventListener("click", () => {
  invoke("open_url", { url: "https://console.groq.com/keys" });
});

loadConfig();
