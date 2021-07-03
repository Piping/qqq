<template>
    <q-page class="flex flex-center q-gutter-md">
    <q-btn push color="purple"
           label="Create a Game"
           @click="createGame()"
    />
    <q-btn push color="primary"
           label="Join a Game"
           @click="enterPin = true;"
    />
    <q-dialog v-model="enterPin">
      <q-card>
        <q-card-section class="row items-center">
          <span class="q-ml-sm">Use the four digit number that is shared with you</span>
        </q-card-section>
        <q-card-section class="row justify-center">
          <q-input v-model="text" :dense="dense">
            <template v-slot:prepend>
              <q-icon name="lock" />
            </template>
            <template v-slot:append>
              <q-btn push color="primary" label="GO" @click="pinSubmit" />
            </template>
          </q-input>
        </q-card-section>
        <q-card-actions align="right">
          <q-btn flat label="Cancel" color="primary" v-close-popup />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </q-page>
</template>

<script>
import { defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'PageIndex',
  setup() {
    const enterPin = ref(false);
    function createGame() {
       this.$router.push({ path: "/game" });
    }
    return {
      enterPin,
      createGame,
    }
  }
})
</script>
