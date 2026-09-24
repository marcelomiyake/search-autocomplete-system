import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'
import App from './App.vue'

describe('App', () => {
  it('renders the search experience and its local-demo context', () => {
    const wrapper = mount(App)

    expect(wrapper.get('h1').text()).toContain('Find what you’re')
    expect(wrapper.get('h1').text()).toContain('looking for.')
    expect(wrapper.text()).toContain('Local demo environment')
    expect(wrapper.get('[aria-label="How suggestions work"]').text()).toContain('Ranked by what people search')
    expect(wrapper.text()).toContain('Try “ru” or “the sky is” to explore demo suggestions.')
    expect(wrapper.get('input[type="search"]').attributes('maxlength')).toBe('50')
  })
})
