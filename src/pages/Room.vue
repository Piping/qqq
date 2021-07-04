<template>
  <q-page class="flex flex-center">
    <div class="q-pa-md">
    <div style="max-width: 980px; min-width:320px; width: 80vw; ">
      <!-- one left two right layout with flex -->
      <div class="fit row wrap justify-center items-start content-start"
           style="overflow: auto; max-height:85vh;"
      >
        <!-- left -->
        <div class="col-12 col-md-6 q-my-xs"
        >
          <q-card class="q-pa-md">
              <q-img :src="game_img"
                  spinner-color="white"
                  sizes='(max-width: 320px) 280px, 440px'
                  class="shake-rotate shake-rotate--hover"
              />
            <q-card-separator />
            <q-card-section>
               <q-item>
                 <q-item-section>
                   <q-item-label caption class="text-center">Game</q-item-label>
                   <q-item-label class="text-center text-h4"> Friday Happy Hour </q-item-label>
                 </q-item-section>
               </q-item>
               <q-item>
                 <q-item-section>
                   <q-item-label class="text-center text-h4"> 20 </q-item-label>
                   <q-item-label caption class="text-center"> Questions </q-item-label>
                 </q-item-section>
                 <q-item-section>
                   <q-item-label class="text-center text-h4"> 3 </q-item-label>
                   <q-item-label caption class="text-center"> Players </q-item-label>
                 </q-item-section>
               </q-item>
            </q-card-section>
          </q-card>
        </div>
        <!-- right -->
        <div class="col-12 col-md-6 q-my-xs"
        >
          <div class="fit column wrap justify-center items-center content-center">
            <div class="col-12">
            <q-table
              title="GameBoard"
              :rows="rows"
              :columns="columns"
              :separator="seperator"
              row-key="name"
              style="max-width: 480px; min-width:320px; width: 35vw;"
            />
            </div>
            <div class="col-12">
              <h2> place holder for url, qrcode </h2>
            </div>
          </div>
        </div>
      </div>
    </div>
    </div>
  </q-page>
</template>

<script>
import { defineComponent, ref } from 'vue';
import game_img from 'src/assets/troll-face.svg';
const rows = [
  {
    user_name: 'Robin',
    user_id: 159,
    user_rank: 1,
    user_ready: 1,
  },
  {
    user_name: 'Steve',
    user_id: 159,
    user_rank: 2,
    user_ready: 1,
  },
  {
    user_name: 'PK',
    user_id: 159,
    user_rank: 3,
    user_ready: 0,
  },
  {
    user_name: 'Sayan',
    user_id: 159,
    user_rank: 3,
    user_ready: 0,
  },
  {
    user_name: 'Yogesh',
    user_id: 159,
    user_rank: 3,
    user_ready: 0,
  },
  {
    user_name: 'Daxay',
    user_id: 159,
    user_rank: 3,
    user_ready: 0,
  },
]
const columns = [
  { name: 'rank', align: 'left', label: 'Rank', field: 'user_rank', sortable: true },
  {
    name: 'name',
    required: true,
    label: 'Names',
    align: 'left',
    field: row => row.user_name,
    format: val => `${val}`,
    sortable: true
  },
  {
    name: 'readyness',
    label: 'Ready',
    field: 'user_ready',
    sortable: true,
    format: val => val ? 'Go': 'Not yet' ,
  }
]
const seperator = 'none';
export default defineComponent({
  name: 'GameRoom',
  setup() {
    return {
      rows,
      columns,
      seperator,
      game_img,
    }
  }
})
</script>
