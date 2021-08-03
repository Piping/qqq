<template>
  <router-view />
</template>
<script>
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'App'
})
</script>
<style>
/* Works on Firefox */
* {
  scrollbar-width: thin;
  scrollbar-color: blue orange;
}

/* Works on Chrome, Edge, and Safari */
*::-webkit-scrollbar {
  width: 3px;
}

*::-webkit-scrollbar-track {
  background: auto;
}

*::-webkit-scrollbar-thumb {
  background-color: blue;
  border-radius: 3px;
  border: 3px solid white;
}
</style>
