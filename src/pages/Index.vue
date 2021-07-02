<template>
  <div class="bg-area" >
    <!-- animated background with css -->
    <!-- put divs before page so button can be on top of background box-->
    <div class="circles">
       <div></div>
       <div></div>
       <div></div>
       <div></div>
       <div></div>
       <div></div>
       <div></div>
       <div></div>
       <div></div>
       <div></div>
    </div>
    <q-page class="flex flex-center q-gutter-md">
    <q-btn push color="purple"
           label="Create a Game"
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
          <q-btn flat label="Cancel" color="primary" v-close-popup="cancelEnabled" :disable="!cancelEnabled" />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </q-page>
  </div>
</template>

<script>
import { defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'PageIndex',
  setup() {
    const enterPin = ref(false);
    return {
      enterPin,
    }
  }
})
</script>

<style>
/* source: https://codepen.io/mohaiman/pen/MQqMyo */
.bg-area{
    background: #4e54c8;
    background: -webkit-linear-gradient(to left, #8f94fb, #4e54c8);
    margin-top: 15px;
}
.circles{
    position: absolute;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    pointer-events: none;
}

.circles div{
    position: absolute;
    display: block;
    list-style: none;
    width: 20px;
    height: 20px;
    background: rgba(255, 255, 255, 0.2);
    animation: animate 25s linear infinite;
    bottom: -150px;
}

.circles div:nth-child(1){
    left: 25%;
    width: 80px;
    height: 80px;
    animation-delay: 0s;
    animation-duration: 15s;
}


.circles div:nth-child(2){
    left: 10%;
    width: 20px;
    height: 20px;
    animation-delay: 2s;
    animation-duration: 12s;
}

.circles div:nth-child(3){
    left: 70%;
    width: 20px;
    height: 20px;
    animation-delay: 4s;
    animation-duration: 25s;
}

.circles div:nth-child(4){
    left: 40%;
    width: 60px;
    height: 60px;
    animation-delay: 0s;
    animation-duration: 18s;
}

.circles div:nth-child(5){
    left: 65%;
    width: 20px;
    height: 20px;
    animation-delay: 0s;
    animation-duration: 10s;
}

.circles div:nth-child(6){
    left: 75%;
    width: 110px;
    height: 110px;
    animation-delay: 3s;
    animation-duration: 15s;
}

.circles div:nth-child(7){
    left: 35%;
    width: 150px;
    height: 150px;
    animation-delay: 7s;
    animation-duration: 20s;
}

.circles div:nth-child(8){
    left: 50%;
    width: 25px;
    height: 25px;
    animation-delay: 15s;
    animation-duration: 45s;
}

.circles div:nth-child(9){
    left: 20%;
    width: 15px;
    height: 15px;
    animation-delay: 2s;
    animation-duration: 35s;
}

.circles div:nth-child(10){
    left: 85%;
    width: 150px;
    height: 150px;
    animation-delay: 0s;
    animation-duration: 11s;
}

@keyframes animate {
    0%{
        transform: translateY(0) rotate(0deg);
        opacity: 1;
        border-radius: 0;
    }
    100%{
        transform: translateY(-1000px) rotate(720deg);
        opacity: 0;
        border-radius: 50%;
    }
}
</style>
