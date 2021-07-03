<template>
  <q-page class="flex flex-center q-gutter-md">
    <div class="q-pa-md">
    <div class="q-gutter-y-md" style="max-width: 980px; min-width:320px; width: 80vw;">
       <q-card>
        <q-toolbar
          class="bg-purple text-white"
        >
        <q-btn push color="blue-7 q-ma-md" label="Create New Questions" @click="pinSubmit" />
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
            <div class="text-h6">Q1: Who has the golden apple?</div>
            <p class="q-mb-xs"> Lorem ipsum dolor sit amet consectetur adipisicing elit.  Lorem ipsum dolor sit amet consectetur adipisicing elit.
                Lorem ipsum dolor sit amet consectetur adipisicing elit.  Lorem ipsum dolor sit amet consectetur adipisicing elit.
                Lorem ipsum dolor sit amet consectetur adipisicing elit.  Lorem ipsum dolor sit amet consectetur adipisicing elit.
            </p>
            <div class="row fit row wrap justify-center items-start content-start">
              <div class="col-12 col-md-6 q-pa-sm">
                <q-card class="my-card">
                  <q-card-section>
                   Lorem ipsum dolor sit amet consectetur adipisicing elit.
                   Lorem ipsum dolor sit amet consectetur adipisicing elit.
                  </q-card-section>
                </q-card>
              </div>
              <div class="col-12 col-md-6 q-pa-sm">
            <q-card class="my-card">
              <q-card-section>
                Lorem ipsum dolor sit amet consectetur adipisicing elit.
              </q-card-section>
            </q-card>
              </div>
              <div class="col-12 col-md-6 q-pa-sm">
            <q-card class="my-card">
              <q-card-section>
                Lorem ipsum dolor sit amet consectetur adipisicing elit.
              </q-card-section>
            </q-card>
              </div>
              <div class="col-12 col-md-6 q-pa-sm">
            <q-card class="my-card">
              <q-card-section>
                Lorem ipsum dolor sit amet consectetur adipisicing elit.
              </q-card-section>
            </q-card>
              </div>
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
  name: 'PageIndex',
  setup() {

    const listOfQuestions = [
      { name: 'Q1', icon: 'mail', label: 'Mails' },
      { name: 'Q2', icon: 'alarm', label: 'Alarms' },
      { name: 'Q3', icon: 'movie', label: 'Movies' },
      { name: 'Q4', icon: 'photo', label: 'Photos' },
      { name: 'Q5', icon: 'slow_motion_video', label: 'Videos' },
      { name: 'Q6', icon: 'people', label: 'Address Book' }
    ];

    return {
      tab: ref('mails'),
    }
  }
})
</script>
