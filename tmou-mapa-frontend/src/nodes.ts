import {Circle, LatLng, LatLngBoundsExpression, LatLngLiteral, LeafletMouseEvent, Rectangle } from "leaflet";

export interface MapRectangle extends Rectangle {
  setId(id: string): void; 
  getId(): string;
  setLatLng(latLng: LatLngLiteral): void;
  getLatLng(): LatLng;
}

export interface MapCircle extends Circle {
  setId(id: string): void; 
  getId(): string;
}

Circle.include({
  id: null,
  setId: function(id) {this.id = id},
  getId: function() {return this.id}
})

Rectangle.include({
  id: null,
  latLng: LatLng,
  setId: function(id) {this.id = id},
  getId: function() {return this.id},
  setLatLng: function(latLng: LatLngLiteral) {this.latLng = new LatLng(latLng.lat, latLng.lng)},
  getLatLng: function() {return this.latLng},
})

export function circleFactory(coords: LatLngLiteral, id: string, color: string, radius: number, onClickHandler: {(e: LeafletMouseEvent): void}): MapCircle {
  const c = new Circle(coords, {color, radius, bubblingMouseEvents: false, className: 'map__node'}) as MapCircle;
  c.setId(id);
  c.on('click', onClickHandler);

  return c;
}

export function squareFactory(coords: LatLngLiteral, id: string, color: string, onClickHandler: {(e: LeafletMouseEvent): void}): MapRectangle {
  const latLngBounds: LatLngBoundsExpression = [[coords.lat - 0.00005, coords.lng - 0.00008], [coords.lat + 0.00005, coords.lng + 0.00008]];
  const c = new Rectangle(latLngBounds, {color, bubblingMouseEvents: false, className: 'map__node'}) as MapRectangle;
  c.setId(id);
  c.setLatLng(coords);
  c.on('click', onClickHandler);

  return c;
}
