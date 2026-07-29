// Aly Client-Side Hydration Script
// This script handles client-side hydration of server-rendered HTML

(function() {
  'use strict';

  window.Aly = window.Aly || {};
  
  const hydrationQueue = [];
  let isHydrating = false;
  
  window.Aly.hydrate = function(container, componentName, props) {
    const element = typeof container === 'string' 
      ? document.querySelector(container) 
      : container;
      
    if (!element) {
      console.warn('Aly: Container not found for hydration', container);
      return Promise.resolve();
    }
    
    return hydrateElement(element, componentName, props);
  };
  
  window.Aly.hydrateAll = function() {
    const elements = document.querySelectorAll('[data-aly-hydrate]');
    return Promise.all(
      Array.from(elements).map(el => {
        const componentName = el.getAttribute('data-aly-hydrate');
        const props = JSON.parse(el.getAttribute('data-aly-props') || '{}');
        return hydrateElement(el, componentName, props);
      })
    );
  };
  
  async function hydrateElement(element, componentName, props) {
    if (element.hasAttribute('data-aly-hydrated')) {
      return;
    }
    
    const Component = window.AlyComponents?.[componentName];
    if (!Component) {
      await waitForComponent(componentName);
      return hydrateElement(element, componentName, props);
    }
    
    try {
      const instance = new Component();
      
      Object.keys(props).forEach(key => {
        instance[key] = props[key];
      });
      
      if (instance.connectedCallback && typeof instance.connectedCallback === 'function') {
        instance.connectedCallback();
      }
      
      element.setAttribute('data-aly-hydrated', 'true');
      element.__alyInstance = instance;
      
      if (instance.render && typeof instance.render === 'function') {
        const content = instance.render();
        if (content instanceof Node) {
          element.innerHTML = '';
          element.appendChild(content);
        } else if (typeof content === 'string') {
          element.innerHTML = content;
        }
      }
      
      if (instance.afterHydrate && typeof instance.afterHydrate === 'function') {
        instance.afterHydrate();
      }
      
      dispatchEvent(element, 'aly:hydrated', { component: componentName });
      
    } catch (error) {
      console.error('Aly hydration error:', error);
      dispatchEvent(element, 'aly:hydration-error', { error: error.message, component: componentName });
    }
  }
  
  function waitForComponent(name) {
    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        reject(new Error(`Component ${name} not defined within 10s`));
      }, 10000);
      
      const check = setInterval(() => {
        if (window.AlyComponents?.[name]) {
          clearInterval(check);
          clearTimeout(timeout);
          resolve();
        }
      }, 50);
    });
  }
  
  function dispatchEvent(element, eventName, detail) {
    const event = new CustomEvent(eventName, {
      detail,
      bubbles: true,
      composed: true
    });
    element.dispatchEvent(event);
  }
  
  window.Aly.registerComponent = function(name, ComponentClass) {
    window.AlyComponents = window.AlyComponents || {};
    window.AlyComponents[name] = ComponentClass;
  };
  
  window.Aly.createElement = function(tagName, props, ...children) {
    const element = document.createElement(tagName);
    
    Object.keys(props || {}).forEach(key => {
      const value = props[key];
      if (key.startsWith('on') && typeof value === 'function') {
        element.addEventListener(key.slice(2).toLowerCase(), value);
      } else if (key === 'style' && typeof value === 'object') {
        Object.assign(element.style, value);
      } else if (key === 'className') {
        element.className = value;
      } else if (typeof value === 'boolean') {
        if (value) element.setAttribute(key, '');
      } else if (value != null) {
        element.setAttribute(key, value);
      }
    });
    
    children.flat().forEach(child => {
      if (child instanceof Node) {
        element.appendChild(child);
      } else if (child != null) {
        element.appendChild(document.createTextNode(String(child)));
      }
    });
    
    return element;
  };
  
  window.Aly.Fragment = function({ children }) {
    const fragment = document.createDocumentFragment();
    (children || []).flat().forEach(child => {
      if (child instanceof Node) {
        fragment.appendChild(child);
      } else if (child != null) {
        fragment.appendChild(document.createTextNode(String(child)));
      }
    });
    return fragment;
  };
  
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
      window.Aly.hydrateAll().catch(console.error);
    });
  } else {
    window.Aly.hydrateAll().catch(console.error);
  }
  
  window.Aly.version = '0.1.0';
})();