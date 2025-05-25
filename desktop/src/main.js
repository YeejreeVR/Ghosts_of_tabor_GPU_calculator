const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function calculate_gpus() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  const tk = await invoke("calculate_gpus", { gpus: document.getElementById("gpus").value , intelutilitylevel:document.getElementById("intellv").value, generatorlevel:document.getElementById("generatorlevel").value });
  const newtk = tk.split("///")
  document.getElementById("second").textContent = `TK per Second: ${newtk[0]}`
  document.getElementById("minute").textContent = `TK per Minute: ${newtk[1]}`
  document.getElementById("hour").textContent = `TK per Hour: ${newtk[2]}`
  document.getElementById("day").textContent = `TK per Day: ${newtk[3]}`
  document.getElementById("gas").textContent = `TK per Gas Can: ${newtk[4]}`
}




const gpusele = document.getElementById("gpus")
const inteluele = document.getElementById("intellv")
gpusele.onchange = calculate_gpus
inteluele.onchange = calculate_gpus
document.getElementById("generatorlevel").onchange = calculate_gpus
