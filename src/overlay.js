const { listen } = window.__TAURI__.event;

const stage = document.getElementById("stage");
const statusText = document.getElementById("status-text");
const lockBadge = document.getElementById("lock-badge");

function setLocked(locked) {
  stage.classList.toggle("locked", locked);
  lockBadge.hidden = !locked;
  statusText.textContent = locked ? "Locked" : "Prompt";
}

function showRecording(event) {
  stage.classList.add("visible");
  stage.classList.remove("transcribing");
  setLocked(Boolean(event.payload?.locked));
}

function showLocked() {
  stage.classList.add("visible");
  stage.classList.remove("transcribing");
  setLocked(true);
}

function showTranscribing() {
  stage.classList.add("visible", "transcribing");
  stage.classList.remove("locked");
  lockBadge.hidden = true;
  statusText.textContent = "…";
}

function hideOverlay() {
  stage.classList.remove("visible", "transcribing", "locked", "error");
  lockBadge.hidden = true;
  statusText.textContent = "Prompt";
}

let errorTimer = null;

function showError(message) {
  stage.classList.add("visible", "error");
  stage.classList.remove("transcribing", "locked");
  lockBadge.hidden = true;
  statusText.textContent = message;
  clearTimeout(errorTimer);
  errorTimer = setTimeout(hideOverlay, 2800);
}

listen("recording-start", showRecording);
listen("recording-locked", showLocked);
listen("transcribing", showTranscribing);
listen("overlay-hide", hideOverlay);
listen("pipeline-error", (event) => {
  showError(event.payload ?? "Something went wrong");
});
