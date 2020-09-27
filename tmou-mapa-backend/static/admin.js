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
    ***************************************************************************** */function e(t,e,n,o){return new(n||(n=Promise))((function(i,a){function r(t){try{s(o.next(t))}catch(t){a(t)}}function c(t){try{s(o.throw(t))}catch(t){a(t)}}function s(t){var e;t.done?i(t.value):(e=t.value,e instanceof n?e:new n((function(t){t(e)}))).then(r,c)}s((o=o.apply(t,e||[])).next())}))}let n,o;const i=(a="map",r=[49.195,16.609],c=15,n=t.map(a).setView(r,c),s=t.tileLayer("https://mapserver.mapy.cz/turist-m/{z}-{x}-{y}",{attribution:'<img src="/static/img/mapy_outline.png" style="width: 10vh"></a>'}),o&&n.removeLayer(o),o=s,o.addTo(n),o.addTo(n),n);var a,r,c,s;const l=t.layerGroup();l.addTo(i);const d=["#000000aa","#ffd83c","#28d428","#2972ff","#ff2929","#9346ba"];function u(){return e(this,void 0,void 0,(function*(){!function(n){l.clearLayers();for(const e of n){const n=new t.Circle(e.position.latLng,18,{color:d[e.level],fillOpacity:1,interactive:!0});n.bindTooltip(e.teamName),n.addTo(l)}document.getElementById("discover").onclick=t=>e(this,void 0,void 0,(function*(){t.preventDefault();const n=document.getElementById("messageForm"),o=new FormData(n);try{yield function(t,n){return e(this,void 0,void 0,(function*(){const e={recipient_id:parseInt(t.get("recipient").toString(),10),message:{content:t.get("message").toString(),type:t.get("type")}},o=n?"/messages/"+n:"/messages";if(!(yield fetch(o,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify(e)})).ok)throw new Error}))}(o),alert("Zpráva odeslána"),document.getElementById("message").value=null}catch(t){alert("Nepovedlo se odeslat zprávu.")}}))}(yield function(){return e(this,void 0,void 0,(function*(){const t=yield fetch("/admin/positions");return(yield t.json()).map(t=>({teamName:t.team_name,position:{latLng:{lat:t.lat,lng:t.lon},type:"ordinary"},level:t.level}))}))}())}))}(function(){return e(this,void 0,void 0,(function*(){yield u(),setInterval(u,5e3)}))})().then(t=>console.log("Running for admin"))}(L);
//# sourceMappingURL=admin.js.map
