export function elementFromHTML<T>(html: string): T {
  let element = document.createElement("div");
  element.innerHTML = html;
  if (1 == element.children.length) {
    let firstChild = element.children[0];
    element.removeChild(firstChild);
    return firstChild as T;
  } else {
    return element as T;
  }
}

export function bindAttributes(view: any, el: HTMLElement) {
  const bindedProperties = new Set<string>();

  const bindProperty = (el: Element) => {
    if (el.hasAttribute("v-prop")) {
      let propertyName = el.getAttribute("v-prop")!;

      if (bindedProperties.has(propertyName)) {
        throw new Error(`v-prop [${propertyName}] conflicts.`);
      }

      if (!view.hasOwnProperty(propertyName)) {
        throw new Error(`property [${propertyName}] not exist view controller instance.`);
      }

      view[propertyName] = el;
      bindedProperties.add(propertyName);
    }
  }

  const bindListenr = (el: Element) => {
    if (el.hasAttribute("v-evt")) {
      const definitions = el.getAttribute("v-evt")!.split(",");

      for (const definition of  definitions) {
        const [event, methodName] = definition.split(":");
        const [eventName, ...eventModifiers] = event.split(".");

        if (!eventName) {
          throw new Error(`v-evt format is invalid. Must be "<eventName>[.<modifier>[.<modifier>]][:<attributeName>][,<eventName>[.<modifier>[.<modifier>]][:<attributeName>]]". [${el.getAttribute("v-evt")}]`);
          throw new Error(`method [${methodName}] not exist view controller instance.`);
          continue;
        }

        for (let modifier of eventModifiers)  {
          const validModifies = ["prevent", "stop", "passive", "self"];
          if (!validModifies.includes(modifier)) {
            throw new Error(`[${modifier}] is not valid. Valid event modifier name is one of [${validModifies.join(", ")}].`);
          }
        }

        if (methodName && !view[methodName]) {
          throw new Error(`method [${methodName}] not exist view controller instance.`);
        }

        if (methodName && !(typeof view[methodName] === "function")) {
          throw new Error(`method [${methodName}] is not a function.`);
        }

        const prevent = eventModifiers.includes("prevent");
        const stop = eventModifiers.includes("stop");
        const passive = eventModifiers.includes("passive");
        const self = eventModifiers.includes("self");

        el.addEventListener(eventName, (e: Event) => {
          if (!self || e.target === el) {
            if (prevent) {
              e.preventDefault();
            }

            if (stop) {
              e.stopPropagation();
            }

            if (methodName) {
              return view[methodName](e);
            }
          }
        }, { passive });
      }
    }
  }

  const bindRecursive = (el: Element) => {
    bindProperty(el);
    bindListenr(el);

    for (let i = 0; i < el.children.length; ++i) {
      let child = el.children[i];
      bindRecursive(child);
    }
  }

  bindRecursive(el);
}
