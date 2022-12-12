const App=
{
  data() {
    return {
      hed:"这里是Actix的欢迎!",
      name:""
    }
  },
  methods: {
    sethallo(){
        axios.post('./api/echo',{name:this.name}).then(res=>{this.hed="你好,"+res.data["name"]}) 
    }
  }
}
Vue.createApp(App).mount('#app')