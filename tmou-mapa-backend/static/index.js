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
<<<<<<< HEAD
<<<<<<< HEAD
    ***************************************************************************** */function t(e,t,n,o){return new(n||(n=Promise))((function(i,s){function r(e){try{c(o.next(e))}catch(e){s(e)}}function a(e){try{c(o.throw(e))}catch(e){s(e)}}function c(e){var t;e.done?i(e.value):(t=e.value,t instanceof n?t:new n((function(e){e(t)}))).then(r,a)}c((o=o.apply(e,t||[])).next())}))}let n,o;function i(){a(e.tileLayer("https://mapserver.mapy.cz/turist-m/{z}-{x}-{y}",{attribution:'<img src="/static/img/mapy_outline.png" style="width: 10vh"></a>'})),o.addTo(n)}function s(){a(e.tileLayer("https://mapserver.mapy.cz/base-m/{z}-{x}-{y}",{attribution:'<img src="/static/img/mapy_outline.png" style="width: 10vh"></a>'})),o.addTo(n)}function r(){a(e.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",{attribution:'&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}))}function a(e){o&&n.removeLayer(o),o=e,o.addTo(n)}function c(t,n,o,i,s){const r=new e.Circle(t,{color:o,radius:i,bubblingMouseEvents:!1,className:"map__node"});return r.setId(n),r.on("click",s),r}function u(t,n,o,i){const s=[[t.lat-5e-5,t.lng-8e-5],[t.lat+5e-5,t.lng+8e-5]],r=new e.Rectangle(s,{color:o,bubblingMouseEvents:!1,className:"map__node"});return r.setId(n),r.setLatLng(t),r.on("click",i),r}function d(e){return t(this,void 0,void 0,(function*(){const t=e?"/game/"+e:"/game",n=yield fetch(t);if(!n.ok)throw new Error("Team state not working, has game started?");return l(yield n.json())}))}function l(e){const{pois:t,state:n,items:o}=e,i=new Map(t.nodes.map(e=>[e.id,{latLng:{lat:e.y,lng:e.x},type:e.type,data:e.data,tag:e.tag}])),s=new Map(t.ways.map(e=>[e.id,{latLng:e.nodes.map(e=>i.get(e).latLng),tag:e.tag}])),r=p(o.items);return{nodes:i,ways:s,state:n,items:r}}function p(e){return e.map(e=>Object.assign(Object.assign({},e),{timestamp:Date.parse(e.timestamp+"+00:00")}))}e.Circle.include({id:null,setId:function(e){this.id=e},getId:function(){return this.id}}),e.Rectangle.include({id:null,latLng:e.LatLng,setId:function(e){this.id=e},getId:function(){return this.id},setLatLng:function(t){this.latLng=new e.LatLng(t.lat,t.lng)},getLatLng:function(){return this.latLng}});const m={popup_failed_heading:"Bohužel...",popup_failed_search_text:"Na toto místo žádná šifra nevede, zkuste to jinde.",popup_neutral_heading:"No...",popup_neutral_badge_text:"Jste tu správně, ale odznáček už máte.",popup_success_badge_text:"Řešení je správně, získali jste za něj odznáček.",popup_neutral_checkpoint_text:"Teď žádné řešení nedostanete.",popup_neutral_search_text:"Jste tu sice správně, ale už jste tu získali všechno, co šlo.",popup_success_heading:"Hurá!",popup_success_search_text:"Jste tu správně!",popup_action_continue:"Pokračovat",popup_checkpoint_select:"Jaké řešení byste chtěli?",section_badges:"Získané odznáčky",section_messages:"Zprávy",section_puzzles:"Zadání šifer",section_actions:"Akce",action_search:"Hledat",action_skip:"Přeskočit šifru",map_switch:"Změnit mapu",no_puzzles_yet:"Zatím jste si žádné zadání nevyzvedli.",no_messages_yet:"Tady se můžou objevit zprávy od organizátorů.",flash_game_finished:"Hra už skončila. Můžete se procházet po mapě, ale nic už nezískáte.",team:"Přihlášený tým:",error:"Došlo k chybě. Zkuste to znovu a případně kontaktujte organizátory.",error_messages:"Došlo k chybě při získávání zpráv. Obnovte stránku a případně kontaktujte organizátory."},g={tagColors:{Europe:"#0085C7",Africa:"#000000","Europe,Africa":"#000000","Africa,Asia":"#F4C300",Asia:"#F4C300","Asia,Australia":"#009F3D",Australia:"#009F3D","Australia,America":"#DF0024",America:"#DF0024",Sifra11:"#800080"}};const y={section_bonuses:"Zadání bonusů",no_bonuses_yet:"Žádný bonus ještě nebyl zveřejněn."};function f(){return t(this,void 0,void 0,(function*(){!function(e){const t=e.map(({url:e,description:t})=>`<li><a href="${e}" target="_blank">${t}</a>`);t.length>0&&(document.querySelector("#bonuses>#bonuses-list").innerHTML=`<ul>${t.join("")}</ul>`)}(yield function(){return t(this,void 0,void 0,(function*(){try{const e="/game/bonuses",t=yield fetch(e);return t.ok?yield t.json():[]}catch(e){return console.error(e),[]}}))}())}))}const h={error:"Došlo k chybě. Zkuste to znovu a případně kontaktujte organizátory.",skip_confirmation:"Opravdu chcete přeskočit šifru?"};function v(e){return t(this,void 0,void 0,(function*(){let n;try{({allowed:n}=yield function(e){return t(this,void 0,void 0,(function*(){const t=e?`/game/${e}/skip`:"/game/skip",n=yield fetch(t);if(!n.ok)throw new Error("Skip check is not working, is game running?");return yield n.json()}))}(e))}catch(e){alert(h.error),console.error(e)}_(n)}))}function _(e){const t=document.getElementById("skip");e?(t.removeAttribute("disabled"),t.classList.remove("disabled"),t.classList.add("enabled")):(t.setAttribute("disabled","disabled"),t.classList.remove("enabled"),t.classList.add("disabled"))}function k(e,n,o){return t(this,void 0,void 0,(function*(){e(h),yield v(o),setInterval(()=>{v(o)},6e4),document.getElementById("skip").onclick=()=>t(this,void 0,void 0,(function*(){const e=window.confirm(h.skip_confirmation);if(e)try{let{newItems:i}=yield function(e,n){return t(this,void 0,void 0,(function*(){const t=n?`/game/${n}/skip`:"/game/skip",o=yield fetch(t,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify({verified:e})});if(!o.ok)throw new Error("Skip check is not working, is game running?");return yield o.json()}))}(e,o);n(i),_(!1)}catch(e){alert(h.error),console.error(e)}}))}))}const b=(w="map",L=[49.195,16.609],z=15,n=e.map(w).setView(L,z),i(),n);var w,L,z;(function(){var n,o;return t(this,void 0,void 0,(function*(){const a=document.querySelector("body").dataset.secretphrase||null,h=(null===(n=document.querySelector("#bonuses"))||void 0===n?void 0:n.dataset.bonusesenabled)||!1,_=(null===(o=document.querySelector("#skip"))||void 0===o?void 0:o.dataset.skipenabled)||!1,w=[],L=new Map,z=new Set;P(m),A(),setInterval(A,1e4),h&&(yield function(e){return t(this,void 0,void 0,(function*(){e(y),yield f(),setInterval(f,6e4)}))}(P)),_&&(yield k(P,J,a));let{nodes:S,ways:E,state:I,items:j}=yield d(a);const $=S.get(I.position).latLng;let x,T,B=new e.LatLng($.lat,$.lng);const O=window.localStorage.getItem("nodesAndWays");if(O){const e=JSON.parse(O);for(const t of e){N(new Map(JSON.parse(t.nodes)),new Map(JSON.parse(t.ways)))}}function A(){return t(this,void 0,void 0,(function*(){try{!function(e){const t={success:"ok",fail:"wrong",info:"info"};let n="";for(const o of e)n+=`\n        <div class="message ${t[o.type]}">\n        <p>${o.content}</p>\n        <p>${D(o.timestamp)}</p>\n        </div>\n      `;n||(n=`\n        <div>\n          <p>${m.no_messages_yet}</p>\n        </div>\n      `);document.getElementById("messages").innerHTML=n}(yield function(e,n){return t(this,void 0,void 0,(function*(){const t=new URL(e?"/messages/"+e:"/messages",document.location.toString());n&&(t.search=new URLSearchParams([["limit",n.toString()]]).toString());const o=yield fetch(t.toString()),i=yield o.json();if(!o.ok)throw new Error("Discover not working, is game running?");return p(i)}))}(a))}catch(e){alert(m.error_messages),console.error(e)}}))}function M(e,t,n){C(e,`<p>${t}</p>`,n)}function C(e,t,n,o){function i(e){e.keyCode&&27!==e.keyCode||(document.getElementById("popup").classList.remove("popup__visible"),document.getElementById("overlay").classList.remove("overlay__visible"),document.getElementById("popup").classList.add("popup__hidden"),document.getElementById("overlay").classList.add("overlay__hidden"),document.querySelector("#popup .large_badge").classList.value="large_badge",o&&e.target.removeEventListener("click",o),document.querySelector(".popup #continue").removeEventListener("click",i),document.querySelector(".popup #close").removeEventListener("click",i),document.removeEventListener("keyup",i))}document.querySelector(".popup_text>h2").textContent=e,document.querySelector(".popup_text>div").innerHTML=t,document.querySelector("#popup .large_badge").classList.add(n),document.getElementById("popup").classList.remove("popup__hidden"),document.getElementById("overlay").classList.remove("overlay__hidden"),document.getElementById("popup").classList.add("popup__visible"),document.getElementById("overlay").classList.add("overlay__visible"),o&&document.querySelector(".popup #continue").addEventListener("click",o),document.querySelector(".popup #continue").addEventListener("click",i),document.querySelector(".popup #close").addEventListener("click",i),document.addEventListener("keyup",i)}function N(n,o){!function(e,t){10===w.length&&w.shift();w.push({nodes:JSON.stringify(Array.from(e)),ways:JSON.stringify(Array.from(t))}),window.localStorage.setItem("nodesAndWays",JSON.stringify(w))}(n,o),x&&x.setStyle({color:T});for(const[t,n]of o)if(!z.has(t)){const o=g.tagColors[n.tag]||"#0085C766",i=new e.Polyline(n.latLng,{color:o,weight:3,interactive:!1});i.bringToBack(),i.addTo(b),z.add(t)}for(const[e,o]of n.entries()){const n=o.latLng;let i=L.get(e);if(!i){T=g.tagColors[o.tag]||"#0085C766";const s=function(n){return t(this,void 0,void 0,(function*(){yield q(n.target,e)}))};if("checkpoint"===o.type)i=u(n,e,T,s);else{const t="junction"===o.type?6:3;i=c(n,e,T,t,s)}i.addTo(b),i.bringToFront(),L.set(e,i)}B.equals(n)&&(i.setStyle({color:"#ff7b00"}),x=i)}}function q(e,n){return t(this,void 0,void 0,(function*(){B=e.getLatLng();const{nodes:o,ways:i,items:s}=yield function(e,n){return t(this,void 0,void 0,(function*(){const t=n?"/game/"+n:"/game",o=yield fetch(t,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify({nodeId:e})});if(!o.ok)throw new Error("Move team not working, has game started?");return l(yield o.json())}))}(n,a);J(s),N(o,i)}))}function D(e){const t=new Date(e),n=t.getHours(),o=t.getMinutes();return`${n<10?"0":""}${n}:${o<10?"0":""}${o}`}function J(e){const t=e.filter(e=>"puzzles"===e.type||"puzzles-fake"===e.type||"dead"===e.type).sort((e,t)=>e.level-t.level).map(({url:e,description:t})=>`<li><a href="${e}" target="_blank">${t}</a>`),n=e.filter(e=>"badge"===e.type).filter(e=>"invisible"!==e.description).sort((e,t)=>e.timestamp-t.timestamp).map(({name:e,description:t})=>`<div class="badge badge-bonus-1" title="${t}"></div>`).join("");document.getElementById("badges").innerHTML=n,t.length&&(document.querySelector("#puzzles>#puzzles-list").innerHTML=`<ul>${t.join("")}</ul>`)}function P(e){for(const[t,n]of Object.entries(e)){const e=document.querySelector(`[data-translation='${t}']`);e&&(e.textContent=n)}}b.setView(B,17),J(j),N(S,E),document.getElementById("teamName").innerText=I.name,document.getElementById("mapSelectorMO").onclick=i,document.getElementById("mapSelectorMB").onclick=s,document.getElementById("mapSelectorOSM").onclick=r,document.getElementById("discover").onclick=()=>t(this,void 0,void 0,(function*(){try{const{event:e,newItems:n}=yield function(e){return t(this,void 0,void 0,(function*(){const t=e?`/game/${e}/discover`:"/game/discover",n=yield fetch(t);if(!n.ok)throw new Error("Discover not working, is game running?");return yield n.json()}))}(a);switch(e){case"nothing":M(m.popup_failed_heading,m.popup_failed_search_text,"shrug");break;case"badge-found":if(n.length){const{name:e}=n[0];!function(e){M(m.popup_success_heading,m.popup_success_badge_text,e)}(e)}else M(m.popup_neutral_heading,m.popup_neutral_badge_text,"shrug");_&&v();break;case"puzzles-found":n.length?M(m.popup_success_heading,m.popup_success_search_text,"get_puzzle"):M(m.popup_neutral_heading,m.popup_neutral_search_text,"shrug");break;case"checkpoint-start-visited":n.length?function(e,n,o){const i=`<form method='POST' id='skipStartPuzzle'><select name="puzzleName">${n.map(e=>`<option value='${e.name}'>${e.description}</option>`).join("")}</select></form>`;C(e,i,o,e=>t(this,void 0,void 0,(function*(){e.preventDefault();const n=document.getElementById("skipStartPuzzle"),o=new FormData(n);try{J(yield function(e,n){return t(this,void 0,void 0,(function*(){const t=n?`/game/${n}/discover`:"/game/discover",o={puzzleName:e.get("puzzleName").toString()},i=yield fetch(t,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify(o)});if(!i.ok)throw new Error("Skip puzzle doesn't work, has game started?");return yield i.json()}))}(o,a))}catch(e){alert(m.error),console.error(e)}})))}(m.popup_checkpoint_select,n,"get_puzzle"):M(m.popup_neutral_heading,m.popup_neutral_checkpoint_text,"shrug")}let{nodes:o,ways:i,items:s}=yield d(a);J(s),N(o,i)}catch(e){alert(m.error),console.error(e)}})),window.debug_node=()=>{console.info("Actual node ID",x.id),console.info("Actual node coord",B)}}))})().then(e=>console.log("Running"))}(L);
=======
    ***************************************************************************** */function t(e,t,n,o){return new(n||(n=Promise))((function(i,s){function r(e){try{c(o.next(e))}catch(e){s(e)}}function a(e){try{c(o.throw(e))}catch(e){s(e)}}function c(e){var t;e.done?i(e.value):(t=e.value,t instanceof n?t:new n((function(e){e(t)}))).then(r,a)}c((o=o.apply(e,t||[])).next())}))}let n,o;function i(){a(e.tileLayer("https://mapserver.mapy.cz/turist-m/{z}-{x}-{y}",{attribution:'<img src="/static/img/mapy_outline.png" style="width: 10vh"></a>'})),o.addTo(n)}function s(){a(e.tileLayer("https://mapserver.mapy.cz/base-m/{z}-{x}-{y}",{attribution:'<img src="/static/img/mapy_outline.png" style="width: 10vh"></a>'})),o.addTo(n)}function r(){a(e.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",{attribution:'&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}))}function a(e){o&&n.removeLayer(o),o=e,o.addTo(n)}function c(t,n,o,i,s){const r=new e.Circle(t,{color:o,radius:i,bubblingMouseEvents:!1,className:"map__node"});return r.setId(n),r.on("click",s),r}function u(t,n,o,i){const s=[[t.lat-5e-5,t.lng-8e-5],[t.lat+5e-5,t.lng+8e-5]],r=new e.Rectangle(s,{color:o,bubblingMouseEvents:!1,className:"map__node"});return r.setId(n),r.setLatLng(t),r.on("click",i),r}function d(e){return t(this,void 0,void 0,(function*(){const t=e?"/game/"+e:"/game",n=yield fetch(t);if(!n.ok)throw new Error("Team state not working, has game started?");return l(yield n.json())}))}function l(e){const{pois:t,state:n,items:o}=e,i=new Map(t.nodes.map(e=>[e.id,{latLng:{lat:e.y,lng:e.x},type:e.type,data:e.data,tag:e.tag}])),s=new Map(t.ways.map(e=>[e.id,{latLng:e.nodes.map(e=>i.get(e).latLng),tag:e.tag}])),r=p(o.items);return{nodes:i,ways:s,state:n,items:r}}function p(e){return e.map(e=>Object.assign(Object.assign({},e),{timestamp:Date.parse(e.timestamp+"+00:00")}))}e.Circle.include({id:null,setId:function(e){this.id=e},getId:function(){return this.id}}),e.Rectangle.include({id:null,latLng:e.LatLng,setId:function(e){this.id=e},getId:function(){return this.id},setLatLng:function(t){this.latLng=new e.LatLng(t.lat,t.lng)},getLatLng:function(){return this.latLng}});const m={popup_failed_heading:"Bohužel...",popup_failed_search_text:"Na toto místo žádná šifra nevede, zkuste to jinde.",popup_neutral_heading:"No...",popup_neutral_badge_text:"Jste tu správně, ale odznáček už máte.",popup_success_badge_text:"Řešení je správně, získali jste za něj odznáček.",popup_success_teleport_text:"Máte dostatek odznáčků a můžete pokračovat do dalšího levelu.",popup_neutral_checkpoint_text:"Teď žádné řešení nedostanete.",popup_neutral_search_text:"Jste tu sice správně, ale už jste tu získali všechno, co šlo.",popup_success_heading:"Hurá!",popup_success_search_text:"Jste tu správně!",popup_action_continue:"Pokračovat",popup_checkpoint_select:"Jaké řešení byste chtěli?",section_badges:"Získané odznáčky",section_messages:"Zprávy",section_puzzles:"Zadání šifer",section_actions:"Akce",action_search:"Hledat",action_skip:"Přeskočit šifru",map_switch:"Změnit mapu",no_puzzles_yet:"Zatím jste si žádné zadání nevyzvedli.",no_messages_yet:"Tady se můžou objevit zprávy od organizátorů.",flash_game_finished:"Hra už skončila. Můžete se procházet po mapě, ale nic už nezískáte.",team:"Přihlášený tým:",error:"Došlo k chybě. Zkuste to znovu a případně kontaktujte organizátory.",error_messages:"Došlo k chybě při získávání zpráv. Obnovte stránku a případně kontaktujte organizátory."},g={tagColors:{Europe:"#0085C7",Africa:"#000000","Europe,Africa":"#000000","Africa,Asia":"#F4C300",Asia:"#F4C300","Asia,Australia":"#009F3D",Australia:"#009F3D","Australia,America":"#DF0024",America:"#DF0024",Sifra11:"#800080"}};const y={section_bonuses:"Zadání bonusů",no_bonuses_yet:"Žádný bonus ještě nebyl zveřejněn."};function f(){return t(this,void 0,void 0,(function*(){!function(e){const t=e.map(({url:e,description:t})=>`<li><a href="${e}" target="_blank">${t}</a>`);t.length>0&&(document.querySelector("#bonuses>#bonuses-list").innerHTML=`<ul>${t.join("")}</ul>`)}(yield function(){return t(this,void 0,void 0,(function*(){try{const e="/game/bonuses",t=yield fetch(e);return t.ok?yield t.json():[]}catch(e){return console.error(e),[]}}))}())}))}const h={error:"Došlo k chybě. Zkuste to znovu a případně kontaktujte organizátory.",skip_confirmation:"Opravdu chcete přeskočit šifru?"};function v(e){return t(this,void 0,void 0,(function*(){let n;try{({allowed:n}=yield function(e){return t(this,void 0,void 0,(function*(){const t=e?`/game/${e}/skip`:"/game/skip",n=yield fetch(t);if(!n.ok)throw new Error("Skip check is not working, is game running?");return yield n.json()}))}(e))}catch(e){alert(h.error),console.error(e)}_(n)}))}function _(e){const t=document.getElementById("skip");e?(t.removeAttribute("disabled"),t.classList.remove("disabled"),t.classList.add("enabled")):(t.setAttribute("disabled","disabled"),t.classList.remove("enabled"),t.classList.add("disabled"))}function k(e,n,o){return t(this,void 0,void 0,(function*(){e(h),yield v(o),setInterval(()=>{v(o)},6e4),document.getElementById("skip").onclick=()=>t(this,void 0,void 0,(function*(){const e=window.confirm(h.skip_confirmation);if(e)try{let{newItems:i}=yield function(e,n){return t(this,void 0,void 0,(function*(){const t=n?`/game/${n}/skip`:"/game/skip",o=yield fetch(t,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify({verified:e})});if(!o.ok)throw new Error("Skip check is not working, is game running?");return yield o.json()}))}(e,o);n(i),_(!1)}catch(e){alert(h.error),console.error(e)}}))}))}const b=(w="map",z=[49.195,16.609],L=15,n=e.map(w).setView(z,L),i(),n);var w,z,L;(function(){var n,o;return t(this,void 0,void 0,(function*(){const a=document.querySelector("body").dataset.secretphrase||null,h=(null===(n=document.querySelector("#bonuses"))||void 0===n?void 0:n.dataset.bonusesenabled)||!1,_=(null===(o=document.querySelector("#skip"))||void 0===o?void 0:o.dataset.skipenabled)||!1,w=[],z=new Map,L=new Set;P(m),A(),setInterval(A,1e4),h&&(yield function(e){return t(this,void 0,void 0,(function*(){e(y),yield f(),setInterval(f,6e4)}))}(P)),_&&(yield k(P,J,a));let{nodes:S,ways:E,state:I,items:j}=yield d(a);const x=S.get(I.position).latLng;let $,T,B=new e.LatLng(x.lat,x.lng);const O=window.localStorage.getItem("nodesAndWays");if(O){const e=JSON.parse(O);for(const t of e){N(new Map(JSON.parse(t.nodes)),new Map(JSON.parse(t.ways)))}}function A(){return t(this,void 0,void 0,(function*(){try{!function(e){const t={success:"ok",fail:"wrong",info:"info"};let n="";for(const o of e)n+=`\n        <div class="message ${t[o.type]}">\n        <p>${o.content}</p>\n        <p>${D(o.timestamp)}</p>\n        </div>\n      `;n||(n=`\n        <div>\n          <p>${m.no_messages_yet}</p>\n        </div>\n      `);document.getElementById("messages").innerHTML=n}(yield function(e,n){return t(this,void 0,void 0,(function*(){const t=new URL(e?"/messages/"+e:"/messages",document.location.toString());n&&(t.search=new URLSearchParams([["limit",n.toString()]]).toString());const o=yield fetch(t.toString()),i=yield o.json();if(!o.ok)throw new Error("Discover not working, is game running?");return p(i)}))}(a))}catch(e){alert(m.error_messages),console.error(e)}}))}function M(e,t,n){C(e,`<p>${t}</p>`,n)}function C(e,t,n,o){function i(e){e.keyCode&&27!==e.keyCode||(document.getElementById("popup").classList.remove("popup__visible"),document.getElementById("overlay").classList.remove("overlay__visible"),document.getElementById("popup").classList.add("popup__hidden"),document.getElementById("overlay").classList.add("overlay__hidden"),document.querySelector("#popup .large_badge").classList.value="large_badge",o&&e.target.removeEventListener("click",o),document.querySelector(".popup #continue").removeEventListener("click",i),document.querySelector(".popup #close").removeEventListener("click",i),document.removeEventListener("keyup",i))}document.querySelector(".popup_text>h2").textContent=e,document.querySelector(".popup_text>div").innerHTML=t,document.querySelector("#popup .large_badge").classList.add(n),document.getElementById("popup").classList.remove("popup__hidden"),document.getElementById("overlay").classList.remove("overlay__hidden"),document.getElementById("popup").classList.add("popup__visible"),document.getElementById("overlay").classList.add("overlay__visible"),o&&document.querySelector(".popup #continue").addEventListener("click",o),document.querySelector(".popup #continue").addEventListener("click",i),document.querySelector(".popup #close").addEventListener("click",i),document.addEventListener("keyup",i)}function N(n,o){!function(e,t){10===w.length&&w.shift();w.push({nodes:JSON.stringify(Array.from(e)),ways:JSON.stringify(Array.from(t))}),window.localStorage.setItem("nodesAndWays",JSON.stringify(w))}(n,o),$&&$.setStyle({color:T});for(const[t,n]of o)if(!L.has(t)){const o=g.tagColors[n.tag]||"#0085C766",i=new e.Polyline(n.latLng,{color:o,weight:3,interactive:!1});i.bringToBack(),i.addTo(b),L.add(t)}for(const[e,o]of n.entries()){const n=o.latLng;let i=z.get(e);if(!i){T=g.tagColors[o.tag]||"#0085C766";const s=function(n){return t(this,void 0,void 0,(function*(){yield q(n.target,e)}))};if("checkpoint"===o.type)i=u(n,e,T,s);else{const t="junction"===o.type?6:3;i=c(n,e,T,t,s)}i.addTo(b),i.bringToFront(),z.set(e,i)}B.equals(n)&&(i.setStyle({color:"#ff7b00"}),$=i)}}function q(e,n){return t(this,void 0,void 0,(function*(){B=e.getLatLng();const{nodes:o,ways:i,items:s}=yield function(e,n){return t(this,void 0,void 0,(function*(){const t=n?"/game/"+n:"/game",o=yield fetch(t,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify({nodeId:e})});if(!o.ok)throw new Error("Move team not working, has game started?");return l(yield o.json())}))}(n,a);J(s),N(o,i)}))}function D(e){const t=new Date(e),n=t.getHours(),o=t.getMinutes();return`${n<10?"0":""}${n}:${o<10?"0":""}${o}`}function J(e){const t=e.filter(e=>"puzzles"===e.type||"puzzles-fake"===e.type||"dead"===e.type).filter(e=>"invisible"!==e.description).sort((e,t)=>e.level-t.level).map(({url:e,description:t})=>`<li><a href="${e}" target="_blank">${t}</a>`),n=e.filter(e=>"badge"===e.type).sort((e,t)=>e.timestamp-t.timestamp).map(({name:e,description:t})=>`<div class="badge badge-bonus-1" title="${t}"></div>`).join("");document.getElementById("badges").innerHTML=n,t.length&&(document.querySelector("#puzzles>#puzzles-list").innerHTML=`<ul>${t.join("")}</ul>`)}function P(e){for(const[t,n]of Object.entries(e)){const e=document.querySelector(`[data-translation='${t}']`);e&&(e.textContent=n)}}b.setView(B,17),J(j),N(S,E),document.getElementById("teamName").innerText=I.name,document.getElementById("mapSelectorMO").onclick=i,document.getElementById("mapSelectorMB").onclick=s,document.getElementById("mapSelectorOSM").onclick=r,document.getElementById("discover").onclick=()=>t(this,void 0,void 0,(function*(){try{const{event:e,newItems:n}=yield function(e){return t(this,void 0,void 0,(function*(){const t=e?`/game/${e}/discover`:"/game/discover",n=yield fetch(t);if(!n.ok)throw new Error("Discover not working, is game running?");return yield n.json()}))}(a);switch(e){case"nothing":M(m.popup_failed_heading,m.popup_failed_search_text,"shrug");break;case"badge-found":if(n.length){const{name:e}=n[0];console.log(e,n),e.startsWith("teleport")?M(m.popup_success_heading,m.popup_success_teleport_text,"teleport"):function(e){M(m.popup_success_heading,m.popup_success_badge_text,e)}(e)}else M(m.popup_neutral_heading,m.popup_neutral_badge_text,"shrug");_&&v();break;case"puzzles-found":n.length?M(m.popup_success_heading,m.popup_success_search_text,"get_puzzle"):M(m.popup_neutral_heading,m.popup_neutral_search_text,"shrug");break;case"checkpoint-start-visited":n.length?function(e,n,o){const i=`<form method='POST' id='skipStartPuzzle'><select name="puzzleName">${n.map(e=>`<option value='${e.name}'>${e.description}</option>`).join("")}</select></form>`;C(e,i,o,e=>t(this,void 0,void 0,(function*(){e.preventDefault();const n=document.getElementById("skipStartPuzzle"),o=new FormData(n);try{J(yield function(e,n){return t(this,void 0,void 0,(function*(){const t=n?`/game/${n}/discover`:"/game/discover",o={puzzleName:e.get("puzzleName").toString()},i=yield fetch(t,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify(o)});if(!i.ok)throw new Error("Skip puzzle doesn't work, has game started?");return yield i.json()}))}(o,a))}catch(e){alert(m.error),console.error(e)}})))}(m.popup_checkpoint_select,n,"get_puzzle"):M(m.popup_neutral_heading,m.popup_neutral_checkpoint_text,"shrug")}let{items:o}=yield d(a);J(o)}catch(e){alert(m.error),console.error(e)}})),window.debug_node=()=>{console.info("Actual node ID",$.id),console.info("Actual node coord",B)}}))})().then(e=>console.log("Running"))}(L);
>>>>>>> 60fe3a9 (specific teleport badge message)
=======
    ***************************************************************************** */function t(e,t,n,o){return new(n||(n=Promise))((function(i,s){function r(e){try{c(o.next(e))}catch(e){s(e)}}function a(e){try{c(o.throw(e))}catch(e){s(e)}}function c(e){var t;e.done?i(e.value):(t=e.value,t instanceof n?t:new n((function(e){e(t)}))).then(r,a)}c((o=o.apply(e,t||[])).next())}))}let n,o;function i(){a(e.tileLayer("https://mapserver.mapy.cz/turist-m/{z}-{x}-{y}",{attribution:'<img src="/static/img/mapy_outline.png" style="width: 10vh"></a>'})),o.addTo(n)}function s(){a(e.tileLayer("https://mapserver.mapy.cz/base-m/{z}-{x}-{y}",{attribution:'<img src="/static/img/mapy_outline.png" style="width: 10vh"></a>'})),o.addTo(n)}function r(){a(e.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",{attribution:'&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}))}function a(e){o&&n.removeLayer(o),o=e,o.addTo(n)}function c(t,n,o,i,s){const r=new e.Circle(t,{color:o,radius:i,bubblingMouseEvents:!1,className:"map__node"});return r.setId(n),r.on("click",s),r}function u(t,n,o,i){const s=[[t.lat-5e-5,t.lng-8e-5],[t.lat+5e-5,t.lng+8e-5]],r=new e.Rectangle(s,{color:o,bubblingMouseEvents:!1,className:"map__node"});return r.setId(n),r.setLatLng(t),r.on("click",i),r}function l(e){return t(this,void 0,void 0,(function*(){const t=e?"/game/"+e:"/game",n=yield fetch(t);if(!n.ok)throw new Error("Team state not working, has game started?");return d(yield n.json())}))}function d(e){const{pois:t,state:n,items:o}=e,i=new Map(t.nodes.map(e=>[e.id,{latLng:{lat:e.y,lng:e.x},type:e.type,data:e.data,tag:e.tag}])),s=new Map(t.ways.map(e=>[e.id,{latLng:e.nodes.map(e=>i.get(e).latLng),tag:e.tag}])),r=p(o.items);return{nodes:i,ways:s,state:n,items:r}}function p(e){return e.map(e=>Object.assign(Object.assign({},e),{timestamp:Date.parse(e.timestamp+"+00:00")}))}e.Circle.include({id:null,setId:function(e){this.id=e},getId:function(){return this.id}}),e.Rectangle.include({id:null,latLng:e.LatLng,setId:function(e){this.id=e},getId:function(){return this.id},setLatLng:function(t){this.latLng=new e.LatLng(t.lat,t.lng)},getLatLng:function(){return this.latLng}});const m={popup_failed_heading:"Bohužel...",popup_failed_search_text:"Na toto místo žádná šifra nevede, zkuste to jinde.",popup_neutral_heading:"No...",popup_neutral_badge_text:"Jste tu správně, ale odznáček už máte.",popup_success_badge_text:"Řešení je správně, získali jste za něj odznáček.",popup_success_teleport_text:"Máte dostatek odznáčků a můžete pokračovat do dalšího levelu.",popup_neutral_checkpoint_text:"Teď žádné řešení nedostanete.",popup_neutral_search_text:"Jste tu sice správně, ale už jste tu získali všechno, co šlo.",popup_success_heading:"Hurá!",popup_success_search_text:"Jste tu správně!",popup_action_continue:"Pokračovat",popup_checkpoint_select:"Jaké řešení byste chtěli?",section_badges:"Získané odznáčky",section_messages:"Zprávy",section_puzzles:"Zadání šifer",section_actions:"Akce",action_search:"Hledat",action_skip:"Přeskočit šifru",map_switch:"Změnit mapu",no_puzzles_yet:"Zatím jste si žádné zadání nevyzvedli.",no_messages_yet:"Tady se můžou objevit zprávy od organizátorů.",flash_game_finished:"Hra už skončila. Můžete se procházet po mapě, ale nic už nezískáte.",team:"Přihlášený tým:",error:"Došlo k chybě. Zkuste to znovu a případně kontaktujte organizátory.",error_messages:"Došlo k chybě při získávání zpráv. Obnovte stránku a případně kontaktujte organizátory."},g={tagColors:{Europe:"#0085C7",Africa:"#000000","Europe,Africa":"#000000","Africa,Asia":"#F4C300",Asia:"#F4C300","Asia,Australia":"#009F3D",Australia:"#009F3D","Australia,America":"#DF0024",America:"#DF0024",Sifra11:"#800080"}};const y={section_bonuses:"Zadání bonusů",no_bonuses_yet:"Žádný bonus ještě nebyl zveřejněn."};function f(){return t(this,void 0,void 0,(function*(){!function(e){const t=e.map(({url:e,description:t})=>`<li><a href="${e}" target="_blank">${t}</a>`);t.length>0&&(document.querySelector("#bonuses>#bonuses-list").innerHTML=`<ul>${t.join("")}</ul>`)}(yield function(){return t(this,void 0,void 0,(function*(){try{const e="/game/bonuses",t=yield fetch(e);return t.ok?yield t.json():[]}catch(e){return console.error(e),[]}}))}())}))}const h={error:"Došlo k chybě. Zkuste to znovu a případně kontaktujte organizátory.",skip_confirmation:"Opravdu chcete přeskočit šifru?"};function v(e){return t(this,void 0,void 0,(function*(){let n;try{({allowed:n}=yield function(e){return t(this,void 0,void 0,(function*(){const t=e?`/game/${e}/skip`:"/game/skip",n=yield fetch(t);if(!n.ok)throw new Error("Skip check is not working, is game running?");return yield n.json()}))}(e))}catch(e){alert(h.error),console.error(e)}_(n)}))}function _(e){const t=document.getElementById("skip");e?(t.removeAttribute("disabled"),t.classList.remove("disabled"),t.classList.add("enabled")):(t.setAttribute("disabled","disabled"),t.classList.remove("enabled"),t.classList.add("disabled"))}function k(e,n,o){return t(this,void 0,void 0,(function*(){e(h),yield v(o),setInterval(()=>{v(o)},6e4),document.getElementById("skip").onclick=()=>t(this,void 0,void 0,(function*(){const e=window.confirm(h.skip_confirmation);if(e)try{let{newItems:i}=yield function(e,n){return t(this,void 0,void 0,(function*(){const t=n?`/game/${n}/skip`:"/game/skip",o=yield fetch(t,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify({verified:e})});if(!o.ok)throw new Error("Skip check is not working, is game running?");return yield o.json()}))}(e,o);n(i),_(!1)}catch(e){alert(h.error),console.error(e)}}))}))}const b=(w="map",z=[49.195,16.609],L=15,n=e.map(w).setView(z,L),i(),n);var w,z,L;(function(){var n,o;return t(this,void 0,void 0,(function*(){const a=document.querySelector("body").dataset.secretphrase||null,h=(null===(n=document.querySelector("#bonuses"))||void 0===n?void 0:n.dataset.bonusesenabled)||!1,_=(null===(o=document.querySelector("#skip"))||void 0===o?void 0:o.dataset.skipenabled)||!1,w=[],z=new Map,L=new Set;P(m),A(),setInterval(A,1e4),h&&(yield function(e){return t(this,void 0,void 0,(function*(){e(y),yield f(),setInterval(f,6e4)}))}(P)),_&&(yield k(P,J,a));let{nodes:S,ways:E,state:I,items:j}=yield l(a);const $=S.get(I.position).latLng;let x,T,B=new e.LatLng($.lat,$.lng);const O=window.localStorage.getItem("nodesAndWays");if(O){const e=JSON.parse(O);for(const t of e){q(new Map(JSON.parse(t.nodes)),new Map(JSON.parse(t.ways)))}}function A(){return t(this,void 0,void 0,(function*(){try{!function(e){const t={success:"ok",fail:"wrong",info:"info"};let n="";for(const o of e)n+=`\n        <div class="message ${t[o.type]}">\n        <p>${o.content}</p>\n        <p>${D(o.timestamp)}</p>\n        </div>\n      `;n||(n=`\n        <div>\n          <p>${m.no_messages_yet}</p>\n        </div>\n      `);document.getElementById("messages").innerHTML=n}(yield function(e,n){return t(this,void 0,void 0,(function*(){const t=new URL(e?"/messages/"+e:"/messages",document.location.toString());n&&(t.search=new URLSearchParams([["limit",n.toString()]]).toString());const o=yield fetch(t.toString()),i=yield o.json();if(!o.ok)throw new Error("Discover not working, is game running?");return p(i)}))}(a))}catch(e){alert(m.error_messages),console.error(e)}}))}function M(e,t,n,o){C(e,`<p>${t}</p>`,n,null,o)}function C(e,t,n,o,i){function s(e){e.keyCode&&27!==e.keyCode||(document.getElementById("popup").classList.remove("popup__visible"),document.getElementById("overlay").classList.remove("overlay__visible"),document.getElementById("popup").classList.add("popup__hidden"),document.getElementById("overlay").classList.add("overlay__hidden"),document.querySelector("#popup .large_badge").classList.value="large_badge",o&&e.target.removeEventListener("click",o),document.querySelector(".popup #continue").removeEventListener("click",s),document.querySelector(".popup #close").removeEventListener("click",s),document.removeEventListener("keyup",s))}document.querySelector(".popup_text>h2").textContent=e,document.querySelector(".popup_text>div").innerHTML=t,document.querySelector("#popup .large_badge").classList.add(n),i&&(document.querySelector("#popup .label").textContent=i),document.getElementById("popup").classList.remove("popup__hidden"),document.getElementById("overlay").classList.remove("overlay__hidden"),document.getElementById("popup").classList.add("popup__visible"),document.getElementById("overlay").classList.add("overlay__visible"),o&&document.querySelector(".popup #continue").addEventListener("click",o),document.querySelector(".popup #continue").addEventListener("click",s),document.querySelector(".popup #close").addEventListener("click",s),document.addEventListener("keyup",s)}function q(n,o){!function(e,t){10===w.length&&w.shift();w.push({nodes:JSON.stringify(Array.from(e)),ways:JSON.stringify(Array.from(t))}),window.localStorage.setItem("nodesAndWays",JSON.stringify(w))}(n,o),x&&x.setStyle({color:T});for(const[t,n]of o)if(!L.has(t)){const o=g.tagColors[n.tag]||"#0085C766",i=new e.Polyline(n.latLng,{color:o,weight:3,interactive:!1});i.bringToBack(),i.addTo(b),L.add(t)}for(const[e,o]of n.entries()){const n=o.latLng;let i=z.get(e);if(!i){T=g.tagColors[o.tag]||"#0085C766";const s=function(n){return t(this,void 0,void 0,(function*(){yield N(n.target,e)}))};if("checkpoint"===o.type)i=u(n,e,T,s);else{const t="junction"===o.type?6:3;i=c(n,e,T,t,s)}i.addTo(b),i.bringToFront(),z.set(e,i)}B.equals(n)&&(i.setStyle({color:"#ff7b00"}),x=i)}}function N(e,n){return t(this,void 0,void 0,(function*(){B=e.getLatLng();const{nodes:o,ways:i,items:s}=yield function(e,n){return t(this,void 0,void 0,(function*(){const t=n?"/game/"+n:"/game",o=yield fetch(t,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify({nodeId:e})});if(!o.ok)throw new Error("Move team not working, has game started?");return d(yield o.json())}))}(n,a);J(s),q(o,i)}))}function D(e){const t=new Date(e),n=t.getHours(),o=t.getMinutes();return`${n<10?"0":""}${n}:${o<10?"0":""}${o}`}function J(e){const t=e.filter(e=>"puzzles"===e.type||"puzzles-fake"===e.type||"dead"===e.type).filter(e=>"invisible"!==e.description).sort((e,t)=>e.level-t.level).map(({url:e,description:t})=>`<li><a href="${e}" target="_blank">${t}</a>`),n=e.filter(e=>"badge"===e.type).sort((e,t)=>e.timestamp-t.timestamp).map(({name:e,description:t,level:n,timestamp:o})=>`<div class="badge lvl${n}" title="${t}">\n          <span class="time">${D(o)}</span>\n          <span class="label">${t.slice(-2)}</span></div>`).join("");document.getElementById("badges").innerHTML=n,t.length&&(document.querySelector("#puzzles>#puzzles-list").innerHTML=`<ul>${t.join("")}</ul>`)}function P(e){for(const[t,n]of Object.entries(e)){const e=document.querySelector(`[data-translation='${t}']`);e&&(e.textContent=n)}}b.setView(B,17),J(j),q(S,E),document.getElementById("teamName").innerText=I.name,document.getElementById("mapSelectorMO").onclick=i,document.getElementById("mapSelectorMB").onclick=s,document.getElementById("mapSelectorOSM").onclick=r,document.getElementById("discover").onclick=()=>t(this,void 0,void 0,(function*(){try{const{event:e,newItems:n}=yield function(e){return t(this,void 0,void 0,(function*(){const t=e?`/game/${e}/discover`:"/game/discover",n=yield fetch(t);if(!n.ok)throw new Error("Discover not working, is game running?");return yield n.json()}))}(a);switch(e){case"nothing":M(m.popup_failed_heading,m.popup_failed_search_text,"shrug");break;case"badge-found":if(n.length){const{name:e}=n[0];e.startsWith("teleport")?M(m.popup_success_heading,m.popup_success_teleport_text,"get_puzzle"):function(e,t){M(m.popup_success_heading,m.popup_success_badge_text,e,t)}(e,e.slice(-2))}else M(m.popup_neutral_heading,m.popup_neutral_badge_text,"shrug");_&&v();break;case"puzzles-found":n.length?M(m.popup_success_heading,m.popup_success_search_text,"get_puzzle"):M(m.popup_neutral_heading,m.popup_neutral_search_text,"shrug");break;case"checkpoint-start-visited":n.length?function(e,n,o){const i=`<form method='POST' id='skipStartPuzzle'><select name="puzzleName">${n.map(e=>`<option value='${e.name}'>${e.description}</option>`).join("")}</select></form>`;C(e,i,o,e=>t(this,void 0,void 0,(function*(){e.preventDefault();const n=document.getElementById("skipStartPuzzle"),o=new FormData(n);try{J(yield function(e,n){return t(this,void 0,void 0,(function*(){const t=n?`/game/${n}/discover`:"/game/discover",o={puzzleName:e.get("puzzleName").toString()},i=yield fetch(t,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify(o)});if(!i.ok)throw new Error("Skip puzzle doesn't work, has game started?");return yield i.json()}))}(o,a))}catch(e){alert(m.error),console.error(e)}})))}(m.popup_checkpoint_select,n,"get_puzzle"):M(m.popup_neutral_heading,m.popup_neutral_checkpoint_text,"shrug")}let{items:o}=yield l(a);J(o)}catch(e){alert(m.error),console.error(e)}})),window.debug_node=()=>{console.info("Actual node ID",x.id),console.info("Actual node coord",B)}}))})().then(e=>console.log("Running"))}(L);
>>>>>>> 005cddf (show quali badges again)
//# sourceMappingURL=index.js.map
