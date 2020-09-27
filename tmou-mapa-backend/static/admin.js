!function(t){"use strict";
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
    ***************************************************************************** */function e(t,e,n,o){return new(n||(n=Promise))((function(i,r){function a(t){try{s(o.next(t))}catch(t){r(t)}}function c(t){try{s(o.throw(t))}catch(t){r(t)}}function s(t){var e;t.done?i(t.value):(e=t.value,e instanceof n?e:new n((function(t){t(e)}))).then(a,c)}s((o=o.apply(t,e||[])).next())}))}const n=function(e,n,o){const i=t.map(e).setView(n,o);return t.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",{attribution:'&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(i),i}("map",[49.195,16.609],15),o=t.layerGroup();o.addTo(n);const i=["#000000aa","#ffd83c","#28d428","#2972ff","#ff2929","#9346ba"];function r(){return e(this,void 0,void 0,(function*(){!function(n){o.clearLayers();for(const e of n){const n=new t.Circle(e.position.latLng,18,{color:i[e.level],fillOpacity:1,interactive:!0});n.bindTooltip(e.teamName),n.addTo(o)}document.getElementById("discover").onclick=t=>e(this,void 0,void 0,(function*(){t.preventDefault();const n=document.getElementById("messageForm"),o=new FormData(n);try{yield function(t,n){return e(this,void 0,void 0,(function*(){const e={recipient_id:parseInt(t.get("recipient").toString(),10),message:{content:t.get("message").toString(),type:t.get("type")}},o=n?"/messages/"+n:"/messages";if(!(yield fetch(o,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify(e)})).ok)throw new Error}))}(o),alert("Zpráva odeslána"),document.getElementById("message").value=null}catch(t){alert("Nepovedlo se odeslat zprávu.")}}))}(yield function(){return e(this,void 0,void 0,(function*(){const t=yield fetch("/admin/positions");return(yield t.json()).map(t=>({teamName:t.team_name,position:{latLng:{lat:t.lat,lng:t.lon},type:"ordinary"},level:t.level}))}))}())}))}(function(){return e(this,void 0,void 0,(function*(){yield r(),setInterval(r,5e3)}))})().then(t=>console.log("Running for admin"))}(L);
//# sourceMappingURL=admin.js.map
