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
    ***************************************************************************** */function e(t,e,n,a){return new(n||(n=Promise))((function(o,i){function r(t){try{s(a.next(t))}catch(t){i(t)}}function c(t){try{s(a.throw(t))}catch(t){i(t)}}function s(t){var e;t.done?o(t.value):(e=t.value,e instanceof n?e:new n((function(t){t(e)}))).then(r,c)}s((a=a.apply(t,e||[])).next())}))}let n,a;const o=(i="map",r=[49.195,16.609],c=15,n=t.map(i).setView(r,c),s=t.tileLayer("https://mapserver.mapy.cz/turist-m/{z}-{x}-{y}",{attribution:'<img src="/static/img/mapy_outline.png" style="width: 10vh"></a>'}),a&&n.removeLayer(a),a=s,a.addTo(n),a.addTo(n),n);var i,r,c,s;const f=t.layerGroup();f.addTo(o);const l=["#ffffff","#aaaaaa","#555555","#ffff55","#aa00aa","#55ff55","#ff55ff","#aa0000","#aa5500","#aa00aa","#ff55ff","#55ffff","#00aaaa","#0000aa","#aaaaff","#000000"];function d(){return e(this,void 0,void 0,(function*(){!function(n){f.clearLayers();for(const e of n){const n=new t.Circle(e.position.latLng,18,{color:l[e.level],fillOpacity:1,interactive:!0});n.bindTooltip(e.teamName),n.addTo(f)}document.getElementById("discover").onclick=t=>e(this,void 0,void 0,(function*(){t.preventDefault();const n=document.getElementById("messageForm"),a=new FormData(n);try{yield function(t,n){return e(this,void 0,void 0,(function*(){const e={recipient_id:parseInt(t.get("recipient").toString(),10),message:{content:t.get("message").toString(),type:t.get("type")}},a=n?"/messages/"+n:"/messages";if(!(yield fetch(a,{method:"POST",headers:{"Content-Type":"application/json;charset=utf-8"},body:JSON.stringify(e)})).ok)throw new Error}))}(a),alert("Zpráva odeslána"),document.getElementById("message").value=null}catch(t){alert("Nepovedlo se odeslat zprávu.")}}))}(yield function(){return e(this,void 0,void 0,(function*(){const t=yield fetch("/admin/positions");return(yield t.json()).map(t=>({teamName:t.team_name,position:{latLng:{lat:t.lat,lng:t.lon},type:"ordinary"},level:t.level}))}))}())}))}function u(){return e(this,void 0,void 0,(function*(){}))}(function(){return e(this,void 0,void 0,(function*(){yield d(),yield u(),setInterval(d,5e3),setInterval(u,1e4)}))})().then(t=>console.log("Running for admin"))}(L);
//# sourceMappingURL=admin.js.map
