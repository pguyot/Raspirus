(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[405],{5557:function(e,n,t){(window.__NEXT_P=window.__NEXT_P||[]).push(["/",function(){return t(1094)}])},1094:function(e,n,t){"use strict";t.r(n),t.d(n,{default:function(){return h}});var o=t(5893),r=t(9008),i=t.n(r),s=t(1163),l=t(7294),a=Object.defineProperty;function c(e,n=!1){let t=window.crypto.getRandomValues(new Uint32Array(1))[0],o=`_${t}`;return Object.defineProperty(window,o,{value:t=>(n&&Reflect.deleteProperty(window,o),null==e?void 0:e(t)),writable:!1,configurable:!0}),t}async function d(e,n={}){return new Promise((t,o)=>{let r=c(e=>{t(e),Reflect.deleteProperty(window,`_${i}`)},!0),i=c(e=>{o(e),Reflect.deleteProperty(window,`_${r}`)},!0);window.__TAURI_IPC__({cmd:e,callback:r,error:i,...n})})}function u(e,n="asset"){let t=encodeURIComponent(e);return navigator.userAgent.includes("Windows")?`https://${n}.localhost/${t}`:`${n}://localhost/${t}`}function h(){let e=(0,s.useRouter)(),[n,t]=(0,l.useState)(""),r=null;async function a(){e.push("/loading"),console.log("Value = "+n);let t=null;""!=n&&(await d("start_scanner",{path:n,update:!1,dbfile:""}).then(e=>t=e).catch(console.error),console.log(t),null!=t&&t.length>0?e.push("/infected"):e.push("/clean"),console.log("Finished"))}d("list_usb_drives").then(console.log).then(e=>r=e).catch(console.error),console.log("Drives: "+r);let c=()=>{e.push("/info")},u=()=>{e.push("/settings")};return(0,o.jsxs)(o.Fragment,{children:[(0,o.jsx)(i(),{children:(0,o.jsx)("title",{children:"Raspirus"})}),(0,o.jsxs)("main",{children:[(0,o.jsx)("div",{className:"flex justify-end",children:(0,o.jsxs)("button",{onClick:u,type:"button",className:"inline-block px-6 py-2 border-2 m-5 border-mainred text-mainred font-medium text-xs leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out",children:[(0,o.jsx)("i",{className:"pr-1 fa fa-gear"})," SETTINGS"]})}),(0,o.jsx)("div",{className:"p-12 text-center relative rounded-lg",children:(0,o.jsx)("div",{children:(0,o.jsx)("div",{className:"flex justify-center items-center h-full",children:(0,o.jsxs)("div",{className:"w-full",children:[(0,o.jsx)("h1",{className:"font-bold leading-tight text-8xl mt-0 mb-2 text-mainred",children:"RASPIRUS"}),r&&r.length>0?(0,o.jsxs)("select",{placeholder:"Select drive",value:n,onChange:e=>{t(e.target.value)},className:" px-3 py-1.5 text-base font-normal text-gray-700 bg-white w-9/12 border border-solid border-maingreen-light rounded transition ease-in-out focus:text-gray-700 focus:bg-white focus:border-maingreen focus:outline-none",children:[(0,o.jsx)("option",{value:"",disabled:!0,children:"Select drive"}),r&&r.map(e=>(0,o.jsx)("option",{value:e,children:e}))]}):(0,o.jsx)("div",{className:" m-auto px-3 py-1.5 text-base font-normal text-gray-700 bg-white w-9/12 border border-solid border-maingreen-light rounded transition ease-in-out focus:text-gray-700 focus:bg-white focus:border-maingreen focus:outline-none",children:"No drives found. Try refreshing this page later"}),(0,o.jsxs)("div",{className:"mt-5",children:[(0,o.jsx)("button",{onClick:a,type:"button",className:"mr-2 inline-block px-7 py-3 bg-mainred text-white font-medium text-sm leading-snug uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:mainred-dark active:shadow-lg transition duration-150 ease-in-out",children:"START"}),(0,o.jsx)("button",{onClick:c,type:"button",className:"ml-2 inline-block px-7 py-3 border-2 border-mainred text-mainred font-medium text-sm leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out",children:"INFO"})]})]})})})})]})]})}((e,n)=>{for(var t in n)a(e,t,{get:n[t],enumerable:!0})})({},{convertFileSrc:()=>u,invoke:()=>d,transformCallback:()=>c})},1163:function(e,n,t){e.exports=t(880)}},function(e){e.O(0,[774,888,179],function(){return e(e.s=5557)}),_N_E=e.O()}]);