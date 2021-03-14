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
    ***************************************************************************** */function t(e,t,n,o){return new(n||(n=Promise))((function(s,i){function r(e){try{c(o.next(e))}catch(e){i(e)}}function a(e){try{c(o.throw(e))}catch(e){i(e)}}function c(e){var t;e.done?s(e.value):(t=e.value,t instanceof n?t:new n((function(e){e(t)}))).then(r,a)}c((o=o.apply(e,t||[])).next())}))}let n,o;function s(){a(e.tileLayer("https://mapserver.mapy.cz/turist-m/{z}-{x}-{y}",{attribution:'<img src="/static/img/mapy_outline.png" style="width: 10vh"></a>'})),o.addTo(n)}function i(){a(e.tileLayer("https://mapserver.mapy.cz/base-m/{z}-{x}-{y}",{attribution:'<img src="/static/img/mapy_outline.png" style="width: 10vh"></a>'})),o.addTo(n)}function r(){a(e.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",{attribution:'&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}))}function a(e){o&&n.removeLayer(o),o=e,o.addTo(n)}function c(t,n,o,s,i){const r=new e.Circle(t,{color:o,radius:s,bubblingMouseEvents:!1,className:"map__node"});return r.setId(n),r.on("click",i),r}function u(e){return t(this,void 0,void 0,(function*(){const t=e?"/game/"+e:"/game",n=yield fetch(t);if(!n.ok)throw new Error("Team state not working, has game started?");return d(yield n.json())}))}function d(e){const{pois:t,state:n,items:o}=e,s=new Map(t.nodes.map(e=>[e.id,{latLng:{lat:e.y,lng:e.x},type:e.type,data:e.data}])),i=new Map(t.ways.map(e=>[e.id,e.nodes.map(e=>s.get(e).latLng)])),r=p(o.items);return{nodes:s,ways:i,state:n,items:r}}function p(e){return e.map(e=>Object.assign(Object.assign({},e),{timestamp:Date.parse(e.timestamp+"+00:00")}))}e.Circle.include({id:null,setId:function(e){this.id=e},getId:function(){return this.id}});const l={popup_failed_heading:"Bohužel...",popup_failed_search_text:"Tady nikdo není.",popup_neutral_heading:"No...",popup_neutral_badge_text:"Jste tu správně, ale odznáček už máte.",popup_success_badge_text:"Řešení je správně, získali jste za něj odznáček.",popup_neutral_checkpoint_text:"Teď žádné řešení nedostanete.",popup_neutral_search_text:"Jste tu sice správně, ale už jste tu získali všechno, co šlo.",popup_success_heading:"Hurá!",popup_success_search_text:"Jste tu správně!",popup_action_continue:"Pokračovat",popup_checkpoint_select:"Jaké řešení byste chtěli?",section_badges:"Získané odznáčky",section_messages:"Zprávy",section_bonuses:"Zadání bonusů",section_puzzles:"Informace",section_actions:"Akce",action_search:"Hledat",action_skip:"Přeskočit šifru",map_switch:"Změnit mapu",no_bonuses_yet:"Žádný bonus ještě nebyl zveřejněn.",no_puzzles_yet:"Zatím jste žádné informace nezískali.",no_messages_yet:"Tady se můžou objevit zprávy od organizátorů.",flash_game_finished:"Hra už skončila. Můžete se procházet po mapě, ale nic už nezískáte.",team:"Přihlášený tým:",error:"Došlo k chybě. Zkuste to znovu a případně kontaktujte organizátory.",error_messages:"Došlo k chybě při získávání zpráv. Obnovte stránku a případně kontaktujte organizátory.",skip_confirmation:"Opravdu chcete přeskočit šifru?"},m=(y="map",g=[49.195,16.609],f=15,n=e.map(y).setView(g,f),s(),n);var y,g,f;(function(){return t(this,void 0,void 0,(function*(){const n=document.querySelector("body").dataset.secretphrase||null,o=new Map,a=new Set,y=[];!function(e){for(const[t,n]of Object.entries(e)){const e=document.querySelector(`[data-translation='${t}']`);e&&(e.textContent=n)}}(l),z(),setInterval(z,1e4);let{nodes:g,ways:f,state:_,items:v}=yield u(n);const h=g.get(_.position).latLng;let k,w=new e.LatLng(h.lat,h.lng);const b=window.localStorage.getItem("nodesAndWays");if(b){const e=JSON.parse(b);for(const t of e){S(new Map(JSON.parse(t.nodes)),new Map(JSON.parse(t.ways)))}}function z(){return t(this,void 0,void 0,(function*(){try{!function(e){const t={success:"ok",fail:"wrong",info:"info"};let n="";for(const o of e)n+=`\n        <div class="message ${t[o.type]}">\n        <p>${o.content}</p>\n        <p>${I(o.timestamp)}</p>\n        </div>\n      `;n||(n=`\n        <div>\n          <p>${l.no_messages_yet}</p>\n        </div>\n      `);document.getElementById("messages").innerHTML=n}(yield function(e,n){return t(this,void 0,void 0,(function*(){const t=new URL(e?"/messages/"+e:"/messages",document.location.toString());n&&(t.search=new URLSearchParams([["limit",n.toString()]]).toString());const o=yield fetch(t.toString()),s=yield o.json();if(!o.ok)throw new Error("Discover not working, is game running?");return p(s)}))}(n))}catch(e){alert(l.error_messages),console.error(e)}}))}function L(e,t,n){!function(e,t,n,o){document.querySelector(".popup_text>h2").textContent=e,document.querySelector(".popup_text>div").innerHTML=t,document.querySelector("#popup .large_badge").classList.add(n),document.getElementById("popup").classList.remove("popup__hidden"),document.getElementById("overlay").classList.remove("overlay__hidden"),document.getElementById("popup").classList.add("popup__visible"),document.getElementById("overlay").classList.add("overlay__visible"),o&&document.querySelector(".popup #continue").addEventListener("click",o);function s(e){e.keyCode&&27!==e.keyCode||(document.getElementById("popup").classList.remove("popup__visible"),document.getElementById("overlay").classList.remove("overlay__visible"),document.getElementById("popup").classList.add("popup__hidden"),document.getElementById("overlay").classList.add("overlay__hidden"),document.querySelector("#popup .large_badge").classList.value="large_badge",o&&e.target.removeEventListener("click",o),document.querySelector(".popup #continue").removeEventListener("click",s),document.querySelector(".popup #close").removeEventListener("click",s),document.removeEventListener("keyup",s))}document.querySelector(".popup #continue").addEventListener("click",s),document.querySelector(".popup #close").addEventListener("click",s),document.addEventListener("keyup",s)}(e,`<p>${t}</p>`,n)}function S(n,s){!function(e,t){10===y.length&&y.shift();y.push({nodes:JSON.stringify(Array.from(e)),ways:JSON.stringify(Array.from(t))}),window.localStorage.setItem("nodesAndWays",JSON.stringify(y))}(n,s),k&&k.setStyle({color:"#000000b5"});for(const[t,n]of s)if(!a.has(t)){const o=new e.Polyline(n,{color:"#00000066",weight:3,interactive:!1});o.bringToBack(),o.addTo(m),a.add(t)}for(const[e,s]of n.entries()){const n=s.latLng;let i=o.get(e);if(!i){const r=function(n){return t(this,void 0,void 0,(function*(){yield E(n.target,e)}))},a="junction"===s.type?6:3;i=c(n,e,"#000000b5",a,r),i.addTo(m),i.bringToFront(),o.set(e,i)}w.equals(n)&&(i.setStyle({color:"#FFEC01"}),k=i)}}function E(e,o){return t(this,void 0,void 0,(function*(){w=e.getLatLng();const{nodes:s,ways:i,items:r}=yield function(e,n){return t(this,void 0,void 0,(function*(){const t=n?"/game/"+n:"/game",o=yield fetch(t,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify({nodeId:e})});if(!o.ok)throw new Error("Move team not working, has game started?");return d(yield o.json())}))}(o,n);j(r),S(s,i)}))}function I(e){const t=new Date(e),n=t.getHours(),o=t.getMinutes();return`${n<10?"0":""}${n}:${o<10?"0":""}${o}`}function j(e){const t=e.filter(e=>"puzzles"===e.type||"puzzles-fake"===e.type||"dead"===e.type).sort((e,t)=>e.level-t.level).map(({url:e,description:t})=>`<li><a href="${e}" target="_blank">${t}</a>`);t.length&&(document.querySelector("#puzzles>#puzzles-list").innerHTML=`<ul>${t.join("")}</ul>`)}m.setView(w,17),j(v),S(g,f),document.getElementById("teamName").innerText=_.name,document.getElementById("mapSelectorMO").onclick=s,document.getElementById("mapSelectorMB").onclick=i,document.getElementById("mapSelectorOSM").onclick=r,document.getElementById("discover").onclick=()=>t(this,void 0,void 0,(function*(){try{const{event:e,newItems:o}=yield function(e){return t(this,void 0,void 0,(function*(){const t=e?`/game/${e}/discover`:"/game/discover",n=yield fetch(t);if(!n.ok)throw new Error("Discover not working, is game running?");return yield n.json()}))}(n);switch(e){case"nothing":L(l.popup_failed_heading,l.popup_failed_search_text,"shrug");break;case"puzzles-found":o.length?L(l.popup_success_heading,l.popup_success_search_text,"get_puzzle"):L(l.popup_neutral_heading,l.popup_neutral_search_text,"shrug")}let{items:s}=yield u(n);j(s)}catch(e){alert(l.error),console.error(e)}})),window.debug_node=()=>{console.info("Actual node ID",k.id),console.info("Actual node coord",w)}}))})().then(e=>console.log("Running"))}(L);
//# sourceMappingURL=index.js.map
