<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let chat = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const newMsg = target.querySelector('#msg').value;
  target.querySelector("#msg").value = "";
  await my_project_backend.send_msg(newMsg);
  await getMsg();
}

async function getMsg() {
  chat.value = await my_project_backend.get_all();
}
getMsg();
</script>

<template>
  <main>
    <section id="chat">
      <div v-for="msg in chat">
        {{ msg }} 
      </div>
    </section>
    <form action="#" @submit="handleSubmit">
      <input id="msg" type="text" />
      <button type="submit">Send</button>
    </form>
  </main>
</template>
