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
    ***************************************************************************** */function t(e,t,n,o){return new(n||(n=Promise))((function(i,s){function r(e){try{c(o.next(e))}catch(e){s(e)}}function a(e){try{c(o.throw(e))}catch(e){s(e)}}function c(e){var t;e.done?i(e.value):(t=e.value,t instanceof n?t:new n((function(e){e(t)}))).then(r,a)}c((o=o.apply(e,t||[])).next())}))}let n,o;function i(){a(e.tileLayer("https://mapserver.mapy.cz/turist-m/{z}-{x}-{y}",{attribution:'<img src="/static/img/mapy_outline.png" style="width: 10vh"></a>'})),o.addTo(n)}function s(){a(e.tileLayer("https://mapserver.mapy.cz/base-m/{z}-{x}-{y}",{attribution:'<img src="/static/img/mapy_outline.png" style="width: 10vh"></a>'})),o.addTo(n)}function r(){a(e.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",{attribution:'&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}))}function a(e){o&&n.removeLayer(o),o=e,o.addTo(n)}function c(t,n,o,i,s){const r=new e.Circle(t,{color:o,radius:i,bubblingMouseEvents:!1,className:"map__node"});return r.setId(n),r.on("click",s),r}function u(e){return t(this,void 0,void 0,(function*(){const t=e?"/game/"+e:"/game",n=yield fetch(t);if(!n.ok)throw new Error("Team state not working, has game started?");return d(yield n.json())}))}function d(e){const{pois:t,state:n,items:o}=e,i=new Map(t.nodes.map(e=>[e.id,{latLng:{lat:e.y,lng:e.x},type:e.type,data:e.data,tag:e.tag}])),s=new Map(t.ways.map(e=>[e.id,{latLng:e.nodes.map(e=>i.get(e).latLng),tag:e.tag}])),r=l(o.items);return{nodes:i,ways:s,state:n,items:r}}function l(e){return e.map(e=>Object.assign(Object.assign({},e),{timestamp:Date.parse(e.timestamp+"+00:00")}))}e.Circle.include({id:null,setId:function(e){this.id=e},getId:function(){return this.id}});const p={popup_failed_heading:"Bohužel...",popup_failed_search_text:"Na toto místo žádná šifra nevede, zkuste to jinde.",popup_neutral_heading:"No...",popup_neutral_badge_text:"Jste tu správně, ale odznáček už máte.",popup_success_badge_text:"Řešení je správně, získali jste za něj odznáček.",popup_neutral_checkpoint_text:"Teď žádné řešení nedostanete.",popup_neutral_search_text:"Jste tu sice správně, ale už jste tu získali všechno, co šlo.",popup_success_heading:"Hurá!",popup_success_search_text:"Jste tu správně!",popup_action_continue:"Pokračovat",popup_checkpoint_select:"Jaké řešení byste chtěli?",section_badges:"Získané odznáčky",section_messages:"Zprávy",section_bonuses:"Zadání bonusů",section_puzzles:"Zadání šifer",section_actions:"Akce",action_search:"Hledat",action_skip:"Přeskočit šifru",map_switch:"Změnit mapu",no_bonuses_yet:"Žádný bonus ještě nebyl zveřejněn.",no_puzzles_yet:"Zatím jste si žádné zadání nevyzvedli.",no_messages_yet:"Tady se můžou objevit zprávy od organizátorů.",flash_game_finished:"Hra už skončila. Můžete se procházet po mapě, ale nic už nezískáte.",team:"Přihlášený tým:",error:"Došlo k chybě. Zkuste to znovu a případně kontaktujte organizátory.",error_messages:"Došlo k chybě při získávání zpráv. Obnovte stránku a případně kontaktujte organizátory.",skip_confirmation:"Opravdu chcete přeskočit šifru?"},m={tagColors:{Europe:"#0085C7",Africa:"#000000",Asia:"#F4C300",Australia:"#009F3D",America:"#DF0024",Sifra11:"#800080"}},g=(y="map",f=[49.195,16.609],h=15,n=e.map(y).setView(f,h),i(),n);var y,f,h;(function(){return t(this,void 0,void 0,(function*(){const n=document.querySelector("body").dataset.secretphrase||null,o=new Map,a=new Set,y=[];!function(e){for(const[t,n]of Object.entries(e)){const e=document.querySelector(`[data-translation='${t}']`);e&&(e.textContent=n)}}(p),j(),I(),L(),setInterval(j,1e4),setInterval(I,6e4),setInterval(L,6e4);let{nodes:f,ways:h,state:v,items:_}=yield u(n);const k=f.get(v.position).latLng;let w,b,z=new e.LatLng(k.lat,k.lng);const S=window.localStorage.getItem("nodesAndWays");if(S){const e=JSON.parse(S);for(const t of e){T(new Map(JSON.parse(t.nodes)),new Map(JSON.parse(t.ways)))}}function L(){return t(this,void 0,void 0,(function*(){let e;try{({allowed:e}=yield function(e){return t(this,void 0,void 0,(function*(){const t=e?`/game/${e}/skip`:"/game/skip",n=yield fetch(t);if(!n.ok)throw new Error("Skip check is not working, is game running?");return yield n.json()}))}(n))}catch(e){alert(p.error),console.error(e)}E(e)}))}function E(e){const t=document.getElementById("skip");e?(t.removeAttribute("disabled"),t.classList.remove("disabled"),t.classList.add("enabled")):(t.setAttribute("disabled","disabled"),t.classList.remove("enabled"),t.classList.add("disabled"))}function j(){return t(this,void 0,void 0,(function*(){try{!function(e){const t={success:"ok",fail:"wrong",info:"info"};let n="";for(const o of e)n+=`\n        <div class="message ${t[o.type]}">\n        <p>${o.content}</p>\n        <p>${O(o.timestamp)}</p>\n        </div>\n      `;n||(n=`\n        <div>\n          <p>${p.no_messages_yet}</p>\n        </div>\n      `);document.getElementById("messages").innerHTML=n}(yield function(e,n){return t(this,void 0,void 0,(function*(){const t=new URL(e?"/messages/"+e:"/messages",document.location.toString());n&&(t.search=new URLSearchParams([["limit",n.toString()]]).toString());const o=yield fetch(t.toString()),i=yield o.json();if(!o.ok)throw new Error("Discover not working, is game running?");return l(i)}))}(n))}catch(e){alert(p.error_messages),console.error(e)}}))}function I(){return t(this,void 0,void 0,(function*(){!function(e){const t=e.map(({url:e,description:t})=>`<li><a href="${e}" target="_blank">${t}</a>`);t.length>0&&(document.querySelector("#bonuses>#bonuses-list").innerHTML=`<ul>${t.join("")}</ul>`)}(yield function(){return t(this,void 0,void 0,(function*(){try{const e="/game/bonuses",t=yield fetch(e);return t.ok?yield t.json():[]}catch(e){return console.error(e),[]}}))}())}))}function $(e,t,n){x(e,`<p>${t}</p>`,n)}function x(e,t,n,o){function i(e){e.keyCode&&27!==e.keyCode||(document.getElementById("popup").classList.remove("popup__visible"),document.getElementById("overlay").classList.remove("overlay__visible"),document.getElementById("popup").classList.add("popup__hidden"),document.getElementById("overlay").classList.add("overlay__hidden"),document.querySelector("#popup .large_badge").classList.value="large_badge",o&&e.target.removeEventListener("click",o),document.querySelector(".popup #continue").removeEventListener("click",i),document.querySelector(".popup #close").removeEventListener("click",i),document.removeEventListener("keyup",i))}document.querySelector(".popup_text>h2").textContent=e,document.querySelector(".popup_text>div").innerHTML=t,document.querySelector("#popup .large_badge").classList.add(n),document.getElementById("popup").classList.remove("popup__hidden"),document.getElementById("overlay").classList.remove("overlay__hidden"),document.getElementById("popup").classList.add("popup__visible"),document.getElementById("overlay").classList.add("overlay__visible"),o&&document.querySelector(".popup #continue").addEventListener("click",o),document.querySelector(".popup #continue").addEventListener("click",i),document.querySelector(".popup #close").addEventListener("click",i),document.addEventListener("keyup",i)}function T(n,i){!function(e,t){10===y.length&&y.shift();y.push({nodes:JSON.stringify(Array.from(e)),ways:JSON.stringify(Array.from(t))}),window.localStorage.setItem("nodesAndWays",JSON.stringify(y))}(n,i),w&&w.setStyle({color:b});for(const[t,n]of i)if(!a.has(t)){const o=m.tagColors[n.tag]||"#0085C766",i=new e.Polyline(n.latLng,{color:o,weight:3,interactive:!1});i.bringToBack(),i.addTo(g),a.add(t)}for(const[e,i]of n.entries()){const n=i.latLng;let s=o.get(e);if(!s){const r=function(n){return t(this,void 0,void 0,(function*(){yield B(n.target,e)}))},a="junction"===i.type?6:3;b=m.tagColors[i.tag]||"#0085C766",s=c(n,e,b,a,r),s.addTo(g),s.bringToFront(),o.set(e,s)}z.equals(n)&&(s.setStyle({color:"#ff7b00"}),w=s)}}function B(e,o){return t(this,void 0,void 0,(function*(){z=e.getLatLng();const{nodes:i,ways:s,items:r}=yield function(e,n){return t(this,void 0,void 0,(function*(){const t=n?"/game/"+n:"/game",o=yield fetch(t,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify({nodeId:e})});if(!o.ok)throw new Error("Move team not working, has game started?");return d(yield o.json())}))}(o,n);M(r),T(i,s)}))}function O(e){const t=new Date(e),n=t.getHours(),o=t.getMinutes();return`${n<10?"0":""}${n}:${o<10?"0":""}${o}`}function M(e){const t=e.filter(e=>"puzzles"===e.type||"puzzles-fake"===e.type||"dead"===e.type).sort((e,t)=>e.level-t.level).map(({url:e,description:t})=>`<li><a href="${e}" target="_blank">${t}</a>`),n=e.filter(e=>"badge"===e.type).sort((e,t)=>e.timestamp-t.timestamp).map(({name:e,description:t})=>`<div class="badge ${e}" title="${t}"></div>`).join("");document.getElementById("badges").innerHTML=n,t.length&&(document.querySelector("#puzzles>#puzzles-list").innerHTML=`<ul>${t.join("")}</ul>`)}g.setView(z,17),M(_),T(f,h),document.getElementById("teamName").innerText=v.name,document.getElementById("mapSelectorMO").onclick=i,document.getElementById("mapSelectorMB").onclick=s,document.getElementById("mapSelectorOSM").onclick=r,document.getElementById("discover").onclick=()=>t(this,void 0,void 0,(function*(){try{const{event:e,newItems:o}=yield function(e){return t(this,void 0,void 0,(function*(){const t=e?`/game/${e}/discover`:"/game/discover",n=yield fetch(t);if(!n.ok)throw new Error("Discover not working, is game running?");return yield n.json()}))}(n);switch(e){case"nothing":$(p.popup_failed_heading,p.popup_failed_search_text,"shrug");break;case"badge-found":if(o.length){const{name:e}=o[0];!function(e){$(p.popup_success_heading,p.popup_success_badge_text,e)}(e)}else $(p.popup_neutral_heading,p.popup_neutral_badge_text,"shrug");L();break;case"puzzles-found":o.length?$(p.popup_success_heading,p.popup_success_search_text,"get_puzzle"):$(p.popup_neutral_heading,p.popup_neutral_search_text,"shrug");break;case"checkpoint-start-visited":o.length?function(e,o,i){const s=`<form method='POST' id='skipStartPuzzle'><select name="puzzleName">${o.map(e=>`<option value='${e.name}'>${e.description}</option>`).join("")}</select></form>`;x(e,s,i,e=>t(this,void 0,void 0,(function*(){e.preventDefault();const o=document.getElementById("skipStartPuzzle"),i=new FormData(o);try{M(yield function(e,n){return t(this,void 0,void 0,(function*(){const t=n?`/game/${n}/discover`:"/game/discover",o={puzzleName:e.get("puzzleName").toString()},i=yield fetch(t,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify(o)});if(!i.ok)throw new Error("Skip puzzle doesn't work, has game started?");return yield i.json()}))}(i,n))}catch(e){alert(p.error),console.error(e)}})))}(p.popup_checkpoint_select,o,"get_puzzle"):$(p.popup_neutral_heading,p.popup_neutral_checkpoint_text,"shrug")}let{items:i}=yield u(n);M(i)}catch(e){alert(p.error),console.error(e)}})),document.getElementById("skip").onclick=()=>t(this,void 0,void 0,(function*(){const e=window.confirm(p.skip_confirmation);if(e)try{let{newItems:o}=yield function(e,n){return t(this,void 0,void 0,(function*(){const t=n?`/game/${n}/skip`:"/game/skip",o=yield fetch(t,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify({verified:e})});if(!o.ok)throw new Error("Skip check is not working, is game running?");return yield o.json()}))}(e,n);M(o),E(!1)}catch(e){alert(p.error),console.error(e)}})),window.debug_node=()=>{console.info("Actual node ID",w.id),console.info("Actual node coord",z)}}))})().then(e=>console.log("Running"))}(L);
//# sourceMappingURL=index.js.map
