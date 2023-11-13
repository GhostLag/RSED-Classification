<svelte:head>
  <script src="https://maps.googleapis.com/maps/api/js?key=API_KEY&libraries=places&callback=initAutocomplete" async defer></script>
  <script lang="ts">
    let autocomplete;

    function initAutocomplete() {
      autocomplete = new google.maps.places.Autocomplete(
      document.getElementById("autocomplete"),
      {
        types: ['address'],
        componentRestrictions: {'country': ['AU']},
        fields: ['address_component']
      });

      autocomplete.addListener("place_changed", onPlaceChanged);
    }

    function onPlaceChanged() {
      var place = autocomplete.getPlace();

      if (!place.geometry) {
        document.getElementById("place").innerText = JSON.stringify(place.address_components);
      }
    }
  </script>
</svelte:head>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let address = "";
  let condition = "Placeholder";

  async function classification(){
    await new Promise(r => setTimeout(r, 900));
    address = document.getElementById("place").innerText;
    condition = await invoke("classification", { address });
  }
</script>

<main class="container">
  <h1>RSED Classification</h1>
  <span>
  2018
  <label class="switch">
    <input type="checkbox">
    <span class="slider"></span>
  </label>
  2021
  </span>
  <input id="autocomplete" placeholder="Enter address" type="text" on:input={classification}/>
  <p>{condition}</p>
  <p id="place" hidden></p>
</main>

<style>
  /* Tysm https://www.w3schools.com/howto/howto_css_switch.asp */
  /* The switch - the box around the slider */
  .switch {
    position: relative;
    display: inline-block;
    width: 60px;
    height: 34px;
  }

  /* Hide default HTML checkbox */
  .switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  /* The slider */
  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: #F08080;
    -webkit-transition: .4s;
    transition: .4s;
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 26px;
    width: 26px;
    left: 4px;
    bottom: 4px;
    background-color: white;
    -webkit-transition: .4s;
    transition: .4s;
  }

  input:checked + .slider {
    background-color: #87CEFA;
  }

  input:focus + .slider {
    box-shadow: 0 0 1px #2196F3;
  }

  input:checked + .slider:before {
    -webkit-transform: translateX(26px);
    -ms-transform: translateX(26px);
    transform: translateX(26px);
  }
</style>
