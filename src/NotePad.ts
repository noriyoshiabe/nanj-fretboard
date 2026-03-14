import "./NotePad.scss";
import { PitchList } from "./types";
import type { Pitch } from "./types";
import Key from "./Key";

export default class NotePad {
  el: HTMLElement;
  onClick?: (pitch: Pitch) => void;

  constructor() {
    this.el = document.createElement("div") as HTMLElement;
    this.el.classList.add("NotePad");

    PitchList.forEach((pitch: Pitch) => {
      const key = new Key(pitch);
      this.el.appendChild(key.el);

      key.onClick = (key: Key) => {
        this.onClick?.(key.pitch);
      };

      key.el.dataset.pitch = pitch;
      
      if (pitch.includes("#")) {
        key.el.classList.add("--black");
      }
    });
  }
}
