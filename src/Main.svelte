<svelte:head>
  <script
    const key = console.log(import.meta.env.key);
    defer
    async
    src="https://maps.googleapis.com/maps/api/js?key=">
  </script>

  <script>

    function initMap() {
      const input = document.getElementById("autocomplete");
      const options = {
        types: ['address'],
        componentRestrictions: {'country': ['AU']},
        fields: ['place_id', 'name']
      };
      const autocomplete = new google.maps.places.Autocomplete(input, options);
    };

    // const key = console.log(import.meta.env.key);
    /*
    let autocomplete;
    function initAutocomplete() {
      autocomplete = new google.maps.places.Autocomplete(
      document.getElementById('autocomplete')
      )
      {
        types: ['establishment'],
        componentRestrictions: {'country': ['AU']},
        fields: ['place_id', 'geometry', 'name']
      }
    }
    */
  </script>
</svelte:head>

<main class="container">
  <h1>RSED Classification</h1>
  <p>2018</p><p>2021</p>
  <input id="autocomplete" placeholder="Enter address" type="text"/>
</main>
