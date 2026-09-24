import { expect, test } from '@playwright/test'
import { fileURLToPath } from 'node:url'

const screenshotPath = fileURLToPath(new URL('../../../docs/assets/search-autocomplete.png', import.meta.url))

test('suggestions, keyboard selection and submitted-query recording', async ({ page }) => {
  let recorded: { query: string; idempotency_key: string } | undefined
  page.on('request', (request) => {
    if (request.url().includes('/api/v1/query-events')) recorded = request.postDataJSON()
  })
  await page.goto('/')
  const input = page.getByRole('combobox', { name: 'Search' })
  await input.fill('ru')
  await expect(page.getByRole('option').first()).toContainText('rust')
  await input.press('ArrowDown')
  await input.press('Enter')
  await expect(page.getByText('Search recorded for “rust”.', { exact: true })).toBeVisible()
  expect(recorded?.query).toBe('rust')
  expect(recorded?.idempotency_key).toBeTruthy()
  await input.fill('the sky is')
  await expect(page.getByRole('option').first()).toContainText('the sky is blue')
  await expect(page.getByRole('option')).toHaveCount(5)
  await page.screenshot({ path: screenshotPath, fullPage: true })
})

test('empty result and service failure states are visible', async ({ page }) => {
  await page.route('**/api/v1/suggestions?prefix=qz', (route) => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ prefix: 'qz', suggestions: [] }) }))
  await page.route('**/api/v1/suggestions?prefix=fail', (route) => route.fulfill({ status: 503, contentType: 'application/json', body: '{}' }))
  await page.goto('/')
  const input = page.getByRole('combobox', { name: 'Search' })
  await input.fill('qz')
  await expect(page.getByText('No popular searches yet.')).toBeVisible()
  await input.fill('fail')
  await expect(page.getByRole('alert')).toContainText('temporarily unavailable')
})
