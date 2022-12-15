const App=
{
  data() {
    return {
      name:"",
      kaisain:"",
      kaisaout:"",
      offset:0,
    }
  },
  watch: {
    // 每当 offset 改变时，这个函数就会执行
    offset(newoffset, oldoffset) {
      this.getKaisapswd()
    }
  },
  methods: {
    sethallo(){
        axios.post('./api/echo',{name:this.name}).then(res=>{this.hed="你好,"+res.data["name"]}) 
    },
    putoffset(){
      console.log(this.offset)
    },
    getKaisapswd(){
      axios.post('./api/kaisa',{kaisa:this.kaisain,offset:this.offset}).then(res=>{console.log(res);this.kaisaout=res.data["kaisaout"]}) 
    }
  }
}
Vue.createApp(App).mount('#app')