const search = document.querySelector("#search");
const result = document.querySelector("#result");

search.addEventListener("input", function() {
  (async () => {
    result.innerHTML = await (await fetch("/search?q=" + encodeURIComponent(search.value))).text();
  })();
});

const addDesc = document.querySelector("#desc");
const addEqn = document.querySelector("#eqn");
const addBtn = document.querySelector("#add");
const preview = document.querySelector("#preview");

function renderPreview(desc, eqn) {
  preview.innerHTML = `<li>
    <div class="description">
      ${desc}
    </div>
    <div class="equation">
      ${katex.renderToString(eqn, { throwOnError: false })}
    </div>
  </li>`;
}

addDesc.addEventListener("input", function() {
  renderPreview(addDesc.value, addEqn.value);
});
addEqn.addEventListener("input", function() {
  renderPreview(addDesc.value, addEqn.value);
});

addBtn.onclick = function() {
  (async () => {
    await fetch(`/add?desc=${encodeURIComponent(addDesc.value)}&eqn=${encodeURIComponent(addEqn.value)}`, {method:"POST"});
    addDesc.value = "";
    addEqn.value = "";
    renderPreview("", "");
  })();
};
