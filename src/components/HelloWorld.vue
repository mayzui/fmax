<script setup>
import {onMounted, ref} from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { fetch } from '@tauri-apps/api/http';
import { readBinaryFile, BaseDirectory } from '@tauri-apps/api/fs';


defineProps({
  msg: String
})

const count = ref(0)

const customFun = ()=>{
  const a = 0.07
  const b = 100
   invoke('custom_command',{a:a,b:b}).then((c)=>{
     console.log(c)
   })

}

const initProcess = async ()=>{
  await invoke('init_process').then((res)=>{
    console.log(res)
  });

   await listen('event-name', (event) => {
    console.log(event);
  });

}

const getWeather = async ()=>{
  const response = await fetch('https://v0.yiketianqi.com/api?unescape=1&version=v91&appid=43656176&appsecret=I42og6Lm&ext=&cityid=&city=%E9%87%8D%E5%BA%86', {
    method: 'GET',
    timeout: 3000,
  }).then((res)=>{
    console.log(res)
  });
}

const readImg = async  ()=>{
  const contents = await readBinaryFile('./images/32x32.png', { dir: BaseDirectory.Resource });
  console.log(contents)
}

onMounted(()=>{
  setTimeout(function(){
    invoke('close_splashscreen');
  },5000)
})
</script>

<template>
<button @click="customFun">要调用rust方法</button>
<button @click="initProcess">开启事件</button>
<button @click="getWeather">获取天气</button>
<button @click="readImg">读取图片</button>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
