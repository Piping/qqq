<template>
  <q-page class="flex flex-center q-gutter-md">
    <div class="q-pa-md">
    <div class="q-gutter-y-md" style="max-width: 980px; min-width:320px; width: 80vw;">
       <q-card>
        <q-toolbar
          class="bg-purple text-white"
        >
        <q-btn push color="blue-7 q-ma-md" label="New Question" @click="pinSubmit()" />
        <q-btn push color="green-7 q-ma-md" label="Start Game" @click="startGame()" />
        <q-tabs
          v-model="tab"
          dense
          shrink
          narrow-indicator
          align="justify"
        >
          <q-tab name="mails" label="Mails" />
          <q-tab name="alarms" label="Alarms" />
          <q-tab name="movies" label="Movies" />
        </q-tabs>
        </q-toolbar>
        <q-separator />

        <q-tab-panels v-model="tab" animated  class="bg-purple-1 text-center">
          <q-tab-panel name="mails" style="max-height:65vh;">
            <div class="text-h6">
              <p v-html="listOfQuestions[0].question_header"></p>
              <q-popup-edit
                v-model="listOfQuestions[0].question_header"
                :cover="false" v-slot="scope"
                anchor="center middle"
                max-width="720px"
                auto-save
                buttons
              >
               <q-editor
                v-model="scope.value"
                min-height="5rem"
                content-class="bg-purple-1"
                autofocus
                @keyup.enter.stop
              />
              </q-popup-edit>
            </div>
            <div class="q-mb-xs">
              <p v-html="listOfQuestions[0].question_text"></p>
              <q-popup-edit
                v-model="listOfQuestions[0].question_text"
                :cover="false" v-slot="scope"
                anchor="center middle"
                max-width="720px"
                auto-save
                buttons
              >
               <q-editor
                v-model="scope.value"
                content-class="bg-purple-1"
                min-height="5rem"
                autofocus
                @keyup.enter.stop
              />
              </q-popup-edit>
            </div>
            <div class="row fit row wrap justify-center items-start content-start">
              <template v-for="option in listOfQuestions[0].answer_option" :key="option.order">
                <div class="col-12 col-md-6 q-pa-sm">
                  <q-card class="my-card">
                    <q-card-section v-html="option.content"/>
                    <q-popup-edit
                      v-model="option.content"
                      :cover="false" v-slot="scope"
                      anchor="center middle"
                      max-width="720px"
                      auto-save
                      buttons
                    >
                     <q-editor
                      v-model="scope.value"
                      content-class="bg-purple-1"
                      min-height="5rem"
                      autofocus
                      @keyup.enter.stop
                    />
                    </q-popup-edit>
                  </q-card>
                </div>
              </template>
            </div>
            <q-page-scroller position="bottom-right" :scroll-offset="150" :offset="[18, 18]">
              <q-btn fab icon="keyboard_arrow_up" color="accent" />
            </q-page-scroller>
          </q-tab-panel>

          <q-tab-panel name="alarms">
            <div class="text-h6">Alarms</div>
            Lorem ipsum dolor sit amet consectetur adipisicing elit.
          </q-tab-panel>

          <q-tab-panel name="movies">
            <div class="text-h6">Movies</div>
            Lorem ipsum dolor sit amet consectetur adipisicing elit.
          </q-tab-panel>
        </q-tab-panels>
      </q-card>
    </div>
  </div>
  </q-page>
</template>

<script>
import { defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'GamePage',
  setup() {

    let listOfQuestions = [
      { name: 'Q1', icon: 'mail', label: 'Mails',
        question_header: 'Q1: Who has the golden apple?',
        question_text: `
          Lorem ipsum dolor sit amet consectetur adipisicing elit.  Lorem ipsum dolor sit amet consectetur adipisicing elit.
          Lorem ipsum dolor sit amet consectetur adipisicing elit.  Lorem ipsum dolor sit amet consectetur adipisicing elit.
          Lorem ipsum dolor sit amet consectetur adipisicing elit.  Lorem ipsum dolor sit amet consectetur adipisicing elit.`,
        answer_option: [
          { order: 'A', content: 'Lorem ipsum dolor sit amet consectetur adipisicing elit.Lorem ipsum dolor sit amet consectetur adipisicing elit.', },
          { order: 'B', content: 'Lorem ipsum dolor sit amet consectetur adipisicing elit.',},
          { order: 'C', content: 'Lorem ipsum dolor sit amet consectetur adipisicing elit.',},
          { order: 'D', content: 'Lorem ipsum dolor sit amet consectetur adipisicing elit.Lorem ipsum dolor sit amet consectetur adipisicing elit.Lorem ipsum dolor sit amet consectetur adipisicing elit.',},
        ],
        answer_at: 0,
      },
      { name: 'Q2', icon: 'alarm', label: 'Alarms' },
      { name: 'Q3', icon: 'movie', label: 'Movies' },
      { name: 'Q4', icon: 'photo', label: 'Photos' },
      { name: 'Q5', icon: 'slow_motion_video', label: 'Videos' },
      { name: 'Q6', icon: 'people', label: 'Address Book' }
    ];

    function startGame() {
      this.$router.push({ name: 'gameroom', params: { gameId: '6666' } });
    }
    return {
      tab: ref('mails'),
      startGame,
      listOfQuestions: ref(listOfQuestions),
    }
  }
})
</script>
