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

    if ("ontouchstart" in document.documentElement) {
      this.el.addEventListener("touchstart", this.onPressed.bind(this));
      this.el.addEventListener("touchend", this.onReleased.bind(this));
    } else {
      this.el.addEventListener("mousedown", this.onPressed.bind(this));
      this.el.addEventListener("mouseup", this.onReleased.bind(this));
    }
  }

  onPressed(e: Event) {
    e.preventDefault();
    this.el.classList.add("--pressed");
    this.onClick?.(this);
  }

  onReleased(e: Event) {
    e.preventDefault();
    this.el.classList.remove("--pressed");
  }
}
