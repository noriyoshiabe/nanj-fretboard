import "./Key.scss";
import type { Pitch } from "./types";

export default class Key {
  el: HTMLElement;
  pitch: Pitch;
  onClick?: (key: Key) => void;

  constructor(pitch: Pitch) {
    this.el = document.createElement("div") as HTMLElement;
    this.el.classList.add("Key");

    this.pitch = pitch;

    this.el.addEventListener("mousedown", this.onPressed.bind(this));
    this.el.addEventListener("touchstart", this.onPressed.bind(this));

    this.el.addEventListener("mouseup", this.onReleased.bind(this));
    this.el.addEventListener("touchend", this.onReleased.bind(this));
  }

  onPressed() {
    this.el.classList.add("--pressed");
    this.onClick?.(this);
  }

  onReleased() {
    this.el.classList.remove("--pressed");
  }
}
