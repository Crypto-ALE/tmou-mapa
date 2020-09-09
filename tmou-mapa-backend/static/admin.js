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
    ***************************************************************************** */function n(t,n,o,e){return new(o||(o=Promise))((function(i,a){function c(t){try{l(e.next(t))}catch(t){a(t)}}function r(t){try{l(e.throw(t))}catch(t){a(t)}}function l(t){var n;t.done?i(t.value):(n=t.value,n instanceof o?n:new o((function(t){t(n)}))).then(c,r)}l((e=e.apply(t,n||[])).next())}))}const o=function(n,o,e){const i=t.map(n).setView(o,e);return t.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",{attribution:'&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(i),i}("map",[49.195,16.609],15),e=["#000000aa","#ffd83c","#28d428","#2972ff","#ff2929","#9346ba"];(function(){return n(this,void 0,void 0,(function*(){const i=yield function(){return n(this,void 0,void 0,(function*(){const t=yield fetch("/admin/positions");return(yield t.json()).map(t=>({teamName:t.team_name,position:{latLng:{lat:t.lat,lng:t.lon},type:"ordinary"},level:t.level}))}))}();for(const n of i){const i=new t.Circle(n.position.latLng,18,{color:e[n.level],fillOpacity:1,interactive:!0});i.bindTooltip(n.teamName),i.addTo(o)}}))})().then(t=>console.log("Running for admin"))}(L);
//# sourceMappingURL=admin.js.map
