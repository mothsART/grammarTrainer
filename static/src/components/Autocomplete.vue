<template>
    <span style="position:relative" v-bind:class="{'open':openSuggestion}">
    <input type="text"
           v-model="search"
           @keydown.enter = 'enter'
           @keydown.down  = 'down'
           @keydown.up    = 'up'
    >
        <ul class="dropdown-menu show" style="width:100%">
            <li v-for="(suggestion, index) in results"
                v-bind:class="{'active': isActive(index)}"
                @click="suggestionClick(index)"
            >
              <a href="#">{{ suggestion }}
              </a>
            </li>
        </ul>
    </span>
</template>

<script>
import lodash from 'lodash'

export default {
  props: {
    value: {
      type: String,
      required: true
    }
  },
  data () {
    return {
      open: false,
      current: 0,
      search: '',
      input: String,
      results: Array
    }
  },
  watch: {
    // à chaque fois que la question change, cette fonction s'exécutera
    search: function (newInput, oldInput) {
      this.Input = newInput
      //verb_autocomplete.request(newInput, this.results)
      // on n'effectue une recherche qui si il y a plus de 3 caractères
      this.getResults(newInput)
    }
  },
  computed: {
    openSuggestion () {
      return true
      return this.results
             && this.results.length !== 0
             && this.open === true
    },
    updateValue (value) {
      if (this.open === false) {
        this.open = true
        this.current = 0
      }
      request(value)
      this.$emit('input', value)
    }
  },
  methods: {
    getResults: _.debounce(
      function (newInput) {
        let self = this
        var myInit = {
            method: 'GET',
            headers: {
            'Content-Type': 'application/json'
            }
        }
        var myRequest = new Request('verbs/input', myInit)
        return fetch(myRequest).then(function(response) {
            response.json().then(function(data) {
                self.results = data.list
            })
        })
      },
      // C'est le nombre de millisecondes que nous attendons
      // pour que l’utilisateur arrête de taper.
      300
    ),
    // When enter pressed on the input
    enter () {
      this.search = this.results[this.current]
      this.open = false
    },
    // When up pressed while suggestions are open
    up () {
      if (this.current > 0) {
        this.current--
      }
    },
    // When up pressed while suggestions are open
    down () {
      if (this.current < this.results.length - 1) {
        this.current++
      }
    },
    // For highlighting element
    isActive (index) {
      return index === this.current
    },
    // When one of the suggestion is clicked
    suggestionClick (index) {
      this.current = index
      this.search = this.results[index]
      this.open = false
    }
  }
}
</script>
