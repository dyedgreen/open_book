const search = document.querySelector("#search");
const result = document.querySelector("#result");

search.addEventListener("input", function() {
  (async () => {
    result.innerHTML = renderList(
      await (await fetch("http://localhost:3030/search?q=" + encodeURIComponent(search.value))).text()
    );
  })();
});

const addDesc = document.querySelector("#desc");
const addEqn = document.querySelector("#eqn");
const addBtn = document.querySelector("#add");
const preview = document.querySelector("#preview");

function renderItem(desc, eqn) {
  const query = search.value.split(/\s+/g).filter(term => term.length > 0);
  for (const term of query)
    desc = desc.replace(new RegExp(term, "gi"), term => `<strong>${term}</strong>`);
  return `<li>
    <div class="description">
      ${desc}
    </div>
    <div class="equation">
      ${katex.renderToString(eqn, { throwOnError: false })}
    </div>
  </li>`;
}
function renderList(json) {
  return JSON.parse(json).map(({description, equation}) => renderItem(description, equation)).join("");
}
function renderPreview(desc, eqn) {
  if (desc.length === 0 && eqn.length === 0) {
    preview.innerHTML = "";
  } else {
    preview.innerHTML = renderItem(desc, eqn);
  }
}

addDesc.addEventListener("input", function() {
  renderPreview(addDesc.value, addEqn.value);
});
addEqn.addEventListener("input", function() {
  renderPreview(addDesc.value, addEqn.value);
});

addBtn.onclick = function() {
  if (addDesc.value.length === 0 || addEqn.value.length === 0) {
    alert("Please input a description and equation!");
    return;
  }
  (async () => {
    await fetch(`/add?desc=${encodeURIComponent(addDesc.value)}&eqn=${encodeURIComponent(addEqn.value)}`, {method:"POST"});
    addDesc.value = "";
    addEqn.value = "";
    renderPreview("", "");
  })();
};
