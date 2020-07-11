import {Circle, LatLngTuple, Layer, LeafletMouseEvent} from "leaflet";

Circle.include({
  name: null,
  setName: function(name) {this.name = name},
  getName: function() {return this.name}
})

export function circleFactory(coords: LatLngTuple, name: string, color: string, radius: number, onClickHandler: {(e: LeafletMouseEvent): void}): Circle {

  const c = new Circle(coords, {color, radius, bubblingMouseEvents: false, className: 'map__node'})
  // @ts-ignore
  c.setName(name);
  c.on('click', onClickHandler);
  c.on('click', (e: LeafletMouseEvent) => {
    c.setStyle({color: 'red'});
  });

  return c;
}
