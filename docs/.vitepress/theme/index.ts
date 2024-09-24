// https://vitepress.dev/guide/custom-theme
import { h } from 'vue'
import type { Theme } from 'vitepress'
import DefaultTheme from 'vitepress/theme'
import './style.css'



const anchor_flash = () => {
  if (typeof window === 'undefined') return;

  const on_click = (el) => {
    const id = el.getAttribute('href');
    const target = document.querySelector(id);

    if (target) {
      try {
        if (target?.firstChild.matches('span.anchor-flash')) return;
      } catch (e) {
        // not span so continue
      }

      const span = document.createElement('span');
      span.className = 'anchor-flash';
      while (target.firstChild) {
        span.appendChild(target.firstChild);
      }
      target.appendChild(span);

      setTimeout(() => {
        while (span.firstChild) {
          target.insertBefore(span.firstChild, span);
        }
        span.remove();
      }, 1250)
    }
  }

  document.body.addEventListener('click', (e) => {
    if (e.target?.matches('a[href^="#"]')) {
      on_click(e.target)
    }
  });
}

export default {
  extends: DefaultTheme,
  Layout: () => {
    return h(DefaultTheme.Layout, null, {
      // https://vitepress.dev/guide/extending-default-theme#layout-slots
    })
  },
  enhanceApp({ app, router, siteData }) {
    anchor_flash();

  }
} satisfies Theme