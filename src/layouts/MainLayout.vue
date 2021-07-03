<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar>
        <q-btn flat round dense class="shake shake-constant shake-constant--hover" icon="assignment_ind" />
        <q-toolbar-title class="shake">
          QQQ!
        </q-toolbar-title>
        <q-btn flat dense icon="help" label="Help"/>
      </q-toolbar>
    </q-header>
    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script>

import { defineComponent, ref } from 'vue'

export default defineComponent({
  name: 'MainLayout',

  components: {
  },

  setup () {
    return {
    }
  }
})
</script>
