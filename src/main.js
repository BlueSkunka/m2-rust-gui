const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function postPokemon() {
  let response = await invoke("post_pokemon", {
    name: document.querySelector('#pokemon-name').value,
    pkType: document.querySelector("#pokemon-type").value,
    gender: document.querySelector("#pokemon-gender").value
  });

  const row = buildElevageRow("", response);

  document.getElementById("elevage-body").innerHTML = document.getElementById("elevage-body").innerHTML.concat(row);
}

async function training() {
  let elevage = await invoke("training_command");

  buildElevageTable(elevage);
}

async function listPokemons() {
  let elevage = await invoke("elevage");

  buildElevageTable(elevage)
}

async function sortByLevel() {
  let elevage = await invoke("sort_level_command");

  buildElevageTable(elevage)
}

async function reproduce() {
  const pokemons = document.querySelectorAll('input[name="pokemon-reproduce"]');

  let selected = [];

  pokemons.forEach((pokemon) => {
    if (pokemon.checked) {
      selected.push(pokemon);
    }
  })

  if (selected.length === 2) {
    console.log("reproduce")
  } else {
    alert('Veuillez choisir 2 pokémons');
  }
}

function buildElevageTable(elevage) {
  let pokemons = "";
  console.log("Pokemons", elevage.pokemons)

  elevage.pokemons.forEach((pokemon) => {
    pokemons = buildElevageRow(pokemons, pokemon);
  });

  document.getElementById("elevage-body").innerHTML = pokemons;
}

function buildElevageRow(pokemons, pokemon) {

  console.log(pokemon);
  pokemons = pokemons.concat("<tr>");
  pokemons = pokemons.concat("<td>");
  pokemons = pokemons.concat(pokemon.name)
  pokemons = pokemons.concat("</td>");
  pokemons = pokemons.concat("<td>");
  pokemons = pokemons.concat(pokemon.level)
  pokemons = pokemons.concat("</td>");
  pokemons = pokemons.concat("<td>");
  pokemons = pokemons.concat(pokemon.pk_type);
  pokemons = pokemons.concat("</td>");
  pokemons = pokemons.concat("<td>");
  pokemons = pokemons.concat(pokemon.experience);
  pokemons = pokemons.concat("</td>");
  pokemons = pokemons.concat("<td>");
  pokemons = pokemons.concat(pokemon.gender);
  pokemons = pokemons.concat("</td>");
  pokemons = pokemons.concat("<td> <input type='checkbox' name='pokemon-reproduce'> </td>")
  pokemons = pokemons.concat("</tr>");

  return pokemons;
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  console.log("Loading elevage....")
  listPokemons().then(r => console.log("Elevage loaded !"))


  document.querySelector("#pokemon-form").addEventListener("submit", (e) => {
    e.preventDefault();

    postPokemon();
  })

  document.querySelector('#elevage-train').addEventListener('click', (e) => {
    e.preventDefault();
    training();
  } )

  document.querySelector('#elevage-sort-level').addEventListener('click', () => {
    sortByLevel()
  })

  document.querySelector('#elevage-reproduce').addEventListener('click', () => {
    reproduce();
  })
});
