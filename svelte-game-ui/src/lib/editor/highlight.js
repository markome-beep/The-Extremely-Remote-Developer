import { styleTags, tags as t } from "@lezer/highlight"

export const highlighting = styleTags({
  Identifier: t.name,
  Number: t.number,
  String: t.string,
  Keyword: t.keyword,
})

