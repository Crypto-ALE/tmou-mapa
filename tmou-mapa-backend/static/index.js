!function(e){"use strict";
/*! *****************************************************************************
    Copyright (c) Microsoft Corporation.

    Permission to use, copy, modify, and/or distribute this software for any
    purpose with or without fee is hereby granted.

    THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
    REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
    AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
    INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
    LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
    OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
    PERFORMANCE OF THIS SOFTWARE.
    ***************************************************************************** */function t(e,t,n,o){return new(n||(n=Promise))((function(i,s){function a(e){try{c(o.next(e))}catch(e){s(e)}}function l(e){try{c(o.throw(e))}catch(e){s(e)}}function c(e){var t;e.done?i(e.value):(t=e.value,t instanceof n?t:new n((function(e){e(t)}))).then(a,l)}c((o=o.apply(e,t||[])).next())}))}function n(t,n,o,i,s){const a=new e.Circle(t,{color:o,radius:i,bubblingMouseEvents:!1,className:"map__node"});return a.setId(n),a.on("click",s),a}function o(e){return t(this,void 0,void 0,(function*(){const t=yield fetch("/game/"+e);return i(yield t.json())}))}function i(e){const{pois:t,state:n,items:o}=e,i=new Map(t.nodes.map(e=>[e.id,{latLng:{lat:e.y,lng:e.x},type:e.type,data:e.data}])),s=t.ways.map(e=>e.nodes.map(e=>i.get(e).latLng)),a=o.items.map(e=>Object.assign(Object.assign({},e),{timestamp:Date.parse(e.timestamp)}));return{nodes:i,ways:s,state:n,items:a}}e.Circle.include({id:null,setId:function(e){this.id=e},getId:function(){return this.id}});const s=function(t,n,o){const i=e.map(t).setView(n,o);return e.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",{attribution:'&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(i),i}("map",[49.195,16.609],15);(function(){return t(this,void 0,void 0,(function*(){const a=document.querySelector("body").dataset.secretphrase;document.getElementById("discover").onclick=()=>t(this,void 0,void 0,(function*(){const{event:e,newItems:n}=yield function(e){return t(this,void 0,void 0,(function*(){const t=yield fetch(`/game/${e}/discover`);return yield t.json()}))}(a);switch(e){case"nothing":m("Bohužel...","Na toto místo žádná šifra nevede, zkuste to jinde.");break;case"badge-found":if(n.length){const{level:e,description:t}=n[0];!function(e,t){const n=e?"lvl"+e:"shrug";document.querySelector("#popup .large_badge > .label").innerHTML=t;m("Hurá!","5"===e?"Gratulujeme k dokončení kvalifikace a budeme se na vás (nejspíš) těšit na startu TMOU.":"Řešení je správně, získali jste za něj odznáček.",n)}(e.toString(),t.slice(-2))}else m("No...","Jste tu správně, ale buď jste tu už byli nebo už jste v jiném levelu, takže nic nedostanete.");break;case"checkpoint-visited":n.filter(e=>"checkpoint"!=e.type).length?m("Hurá!","Organizátoři vám dali nové šifry."):m("Bohužel...","Tentokrát jste od organizátorů nic nedostali.");break}let{items:i}=yield o(a);v(i)}));let{nodes:l,ways:c,state:d,items:r}=yield o(a);const u=l.get(d.position).latLng;let p=new e.LatLng(u.lat,u.lng);function m(e,t,n="shrug"){function o(e){e.keyCode&&27!==e.keyCode||(document.getElementById("popup").classList.remove("popup__visible"),document.getElementById("overlay").classList.remove("overlay__visible"),document.getElementById("popup").classList.add("popup__hidden"),document.getElementById("overlay").classList.add("overlay__hidden"),document.querySelector("#popup .large_badge").classList.value="large_badge",e.target.removeEventListener("click",o),document.removeEventListener("keyup",o))}document.querySelector(".popup_text>h2").textContent=e,document.querySelector(".popup_text>p").innerHTML=t,document.querySelector("#popup .large_badge").classList.add(n),document.getElementById("popup").classList.remove("popup__hidden"),document.getElementById("overlay").classList.remove("overlay__hidden"),document.getElementById("popup").classList.add("popup__visible"),document.getElementById("overlay").classList.add("overlay__visible"),document.querySelector(".popup #continue").addEventListener("click",o),document.addEventListener("keyup",o)}function g(o,i){for(const[e,i]of o.entries()){const o=i.latLng,a=n(o,e,"blue",2,(function(n){return t(this,void 0,void 0,(function*(){yield y(n.target,e)}))}));p.equals(o)&&a.setStyle({color:"salmon"}),a.addTo(s)}for(const t of i){const n=new e.Polyline(t,{color:"black",weight:1,interactive:!1});n.bringToBack(),n.addTo(s)}}function y(e,n){return t(this,void 0,void 0,(function*(){s.setView(e.getLatLng(),s.getZoom()),p=e.getLatLng(),console.debug("Aktuální pozice: "+p.toString(),"Aktuální nodeId: "+d.position);const{nodes:o,ways:l,items:c}=yield function(e,n){return t(this,void 0,void 0,(function*(){const t=yield fetch("/game/"+e,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify({nodeId:n})});return i(yield t.json())}))}(a,n);v(c),g(o,l)}))}function v(e){const t=e.filter(e=>"puzzles"===e.type).sort((e,t)=>e.level-t.level).map(({url:e,level:t})=>`<li><a href="${e}">Level ${t}</a>`),n=e.filter(e=>"badge"===e.type).sort((e,t)=>e.timestamp-t.timestamp).map(({level:e,description:t,timestamp:n})=>`<div class="badge lvl${e}">\n            <span class="time">${function(e){const t=new Date(e),n=t.getHours(),o=t.getMinutes();return`${n<10?"0":""}${n}:${o<10?"0":""}${o}`}(n)}</span>\n            <span class="label">${t.slice(-2)}</span>\n          </div>`).join("");document.getElementById("badges").innerHTML=n,t.length&&(document.querySelector("#puzzles>#puzzles-list").innerHTML=`<ul>${t.join("")}</ul>`)}document.getElementById("teamName").innerText=d.name,s.setView(p,17),console.debug("Aktuální pozice: "+p.toString(),"Aktuální nodeId: "+d.position),v(r),g(l,c)}))})().then(e=>console.log("Running"))}(L);
//# sourceMappingURL=index.js.map
