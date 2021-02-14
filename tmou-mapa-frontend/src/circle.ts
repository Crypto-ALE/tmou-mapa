import {Circle, LatLngExpression, LeafletMouseEvent} from "leaflet";

Circle.include({
  id: null,
  setId: function(id) {this.id = id},
  getId: function() {return this.id}
})

export function circleFactory(coords: LatLngExpression, id: string, color: string, radius: number, onClickHandler: {(e: LeafletMouseEvent): void}): Circle {
  const c = new Circle(coords, {color, radius, bubblingMouseEvents: false, className: 'map__node'})
  // @ts-ignore
  c.setId(id);
  c.on('click', onClickHandler);

  return c;
}
