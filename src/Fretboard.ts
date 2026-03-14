import "./Fretboard.scss";

export default class Fretboard {
  el: HTMLElement;

  constructor() {
    this.el = document.createElement("div") as HTMLElement;
    this.el.classList.add("Fretboard");

    for (let i = 1; i <= 6; i++) {
      const _string = document.createElement("span") as HTMLElement;
      _string.classList.add("string");
      _string.style.setProperty("--number", i.toString());
      this.el.appendChild(_string);
    }

    for (let i = 1; i <= 13; i++) {
      const fret = document.createElement("span") as HTMLElement;
      fret.classList.add("fret");
      fret.style.setProperty("--number", i.toString());
      this.el.appendChild(fret);
      
      switch (i) {
      case 3:
      case 5:
      case 7:
      case 9:
        var dot = document.createElement("span") as HTMLElement;
        dot.classList.add("dot");
        dot.style.setProperty("--fret", i.toString());
        this.el.appendChild(dot);
        break;
      case 12:
        var dot = document.createElement("span") as HTMLElement;
        dot.classList.add("dot");
        dot.classList.add("--top");
        dot.style.setProperty("--fret", i.toString());
        this.el.appendChild(dot);
        var dot = document.createElement("span") as HTMLElement;
        dot.classList.add("dot");
        dot.classList.add("--bottom");
        dot.style.setProperty("--fret", i.toString());
        this.el.appendChild(dot);
        break;
      }
    }
  }
}
