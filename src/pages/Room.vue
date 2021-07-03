<template>
  <q-page class="flex flex-center">
    <div class="q-px-md">
    <div style="max-width: 980px; min-width:320px; width: 80vw; ">
      <!-- one left two right layout with flex -->
      <div class="fit row wrap justify-start items-start content-start" style="overflow: auto; max-height:85vh; ">
        <!-- left -->
        <div class="col-12 col-md-6">
           <h2> left place holder for image, qrcode </h2>
           <h2> left place holder for url, qrcode </h2>
           <h2> left place holder for url, qrcode </h2>
        </div>
        <!-- right -->
        <div class="col-12 col-md-6" >
          <div class="fit column wrap justify-start items-start content-start">
            <div class="col-12">
            <q-table
              title="Players"
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
const rows = [
  {
    user_name: 'Steve',
    user_id: 159,
    user_rank: 1,
    user_ready: 1,
  },
  {
    user_name: 'Robin',
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
]
const columns = [
  { name: 'Order', align: 'left', label: 'Id', field: 'user_rank', sortable: true },
  {
    name: 'name',
    required: true,
    label: 'Names',
    align: 'center',
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
    }
  }
})
</script>
