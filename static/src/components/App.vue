<template>
  <span id="app">
    <autocomplete v-model="selection"></autocomplete>
  </span>
</template>

<script>
import Autocomplete from './Autocomplete'

export default {
  name: 'app',
  components: {
    Autocomplete
  },
  data () {
    return {
      selection: ''
    }
  }
}
</script>

<style>
</style>
