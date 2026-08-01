---
name: Sovereign Trust
colors:
  surface: '#f7f9fc'
  surface-dim: '#d8dadd'
  surface-bright: '#f7f9fc'
  surface-container-lowest: '#ffffff'
  surface-container-low: '#f2f4f7'
  surface-container: '#eceef1'
  surface-container-high: '#e6e8eb'
  surface-container-highest: '#e0e3e6'
  on-surface: '#191c1e'
  on-surface-variant: '#44464e'
  inverse-surface: '#2d3133'
  inverse-on-surface: '#eff1f4'
  outline: '#75777f'
  outline-variant: '#c5c6cf'
  surface-tint: '#4c5e86'
  primary: '#00081e'
  on-primary: '#ffffff'
  primary-container: '#0a1f44'
  on-primary-container: '#7687b2'
  inverse-primary: '#b4c6f4'
  secondary: '#b80d2d'
  on-secondary: '#ffffff'
  secondary-container: '#dc2f42'
  on-secondary-container: '#fffbff'
  tertiary: '#000b04'
  on-tertiary: '#ffffff'
  tertiary-container: '#002615'
  on-tertiary-container: '#2c9969'
  error: '#ba1a1a'
  on-error: '#ffffff'
  error-container: '#ffdad6'
  on-error-container: '#93000a'
  primary-fixed: '#d9e2ff'
  primary-fixed-dim: '#b4c6f4'
  on-primary-fixed: '#041a3f'
  on-primary-fixed-variant: '#34466d'
  secondary-fixed: '#ffdad9'
  secondary-fixed-dim: '#ffb3b2'
  on-secondary-fixed: '#410008'
  on-secondary-fixed-variant: '#920020'
  tertiary-fixed: '#8ff7bf'
  tertiary-fixed-dim: '#73daa4'
  on-tertiary-fixed: '#002112'
  on-tertiary-fixed-variant: '#005233'
  background: '#f7f9fc'
  on-background: '#191c1e'
  surface-variant: '#e0e3e6'
  deep-navy: '#06152F'
  dark-red: '#9F172B'
  warning-gold: '#D99A21'
  border-gray: '#DDE4EC'
  soft-blue-bg: '#EEF3F9'
typography:
  headline-xl:
    fontFamily: Plus Jakarta Sans
    fontSize: 40px
    fontWeight: '700'
    lineHeight: 48px
    letterSpacing: -0.02em
  headline-lg:
    fontFamily: Plus Jakarta Sans
    fontSize: 32px
    fontWeight: '700'
    lineHeight: 40px
    letterSpacing: -0.02em
  headline-md:
    fontFamily: Plus Jakarta Sans
    fontSize: 24px
    fontWeight: '600'
    lineHeight: 32px
  body-lg:
    fontFamily: Inter
    fontSize: 18px
    fontWeight: '400'
    lineHeight: 28px
  body-md:
    fontFamily: Inter
    fontSize: 16px
    fontWeight: '400'
    lineHeight: 24px
  body-sm:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: '400'
    lineHeight: 20px
  label-caps:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: '700'
    lineHeight: 16px
    letterSpacing: 0.05em
  receipt-code:
    fontFamily: JetBrains Mono
    fontSize: 14px
    fontWeight: '500'
    lineHeight: 20px
  headline-lg-mobile:
    fontFamily: Plus Jakarta Sans
    fontSize: 28px
    fontWeight: '700'
    lineHeight: 36px
rounded:
  sm: 0.125rem
  DEFAULT: 0.25rem
  md: 0.375rem
  lg: 0.5rem
  xl: 0.75rem
  full: 9999px
spacing:
  unit: 8px
  container-max: 1280px
  gutter: 24px
  margin-mobile: 16px
  margin-desktop: 48px
---

## Brand & Style

The design system is engineered for the highest levels of civic trust, emphasizing security, officiality, and cultural resonance for the Nepalese electoral process. The brand personality is **Institutional**, **Transparent**, and **Infallible**. It avoids the flighty aesthetics of typical SaaS products in favor of a "Government-Grade" interface that feels as permanent and secure as a physical ballot box.

The visual style is **Corporate / Modern** with a lean toward **High-Contrast Functionalism**. It utilizes a structured grid, clear bilingual hierarchies, and subtle geometric patterns inspired by Nepalese textile motifs to create a sense of national identity without compromising professional clarity. The emotional goal is to provide the voter with a sense of calm confidence and the administrator with a sense of total control.

## Colors

The color palette is anchored by **Navy Blue (#0A1F44)**, representing authority and the stability of the state. **Election Red (#C91F37)** is used sparingly for branding accents and critical call-to-actions, nodding to the national flag of Nepal. 

- **Primary (Navy Blue):** Used for navigation, headers, and primary interactions to signify security.
- **Secondary (Election Red):** Reserved for brand moments and high-priority alerts. 
- **Success (Green):** Specifically calibrated for WCAG AA compliance against light backgrounds to indicate a successfully cast vote.
- **Neutral/Backgrounds:** We utilize a "layered white" approach. The `Soft Blue Background` is used to differentiate the page body from `White` content cards, reducing eye strain during long administrative sessions.

## Typography

This design system employs a dual-font strategy to balance modernity with readability. **Plus Jakarta Sans** provides a friendly yet structured feel for headings. **Inter** is the workhorse for all body copy and bilingual labels, chosen for its exceptional legibility and support for complex scripts. 

For the Nepalese context, **Noto Sans Devanagari** must be paired with Inter to ensure that English and Nepali text maintain a harmonious x-height and visual weight. **JetBrains Mono** is introduced specifically for "Voting Receipts" and "Transaction Hashes" to give users a clear, technical confirmation that their vote has been digitally recorded and is verifiable.

## Layout & Spacing

The layout follows a **Fixed Grid** model for desktop to maintain an "official document" feel, centering the content to focus the voter's attention. A 12-column grid is used for administrative dashboards, while a simplified 6-column centered layout is used for the voting flow to minimize distractions.

- **Desktop (1280px+):** 12 columns, 24px gutters, 48px page margins.
- **Tablet (768px - 1024px):** 8 columns, 16px gutters, 32px page margins.
- **Mobile (<768px):** 4 columns, 16px gutters, 16px page margins.

Spacing follows a strict 8px base unit. Heavy vertical rhythm is used in forms to ensure that labels, input fields, and help text are clearly associated, preventing user error during the voting process.

## Elevation & Depth

To maintain a sense of "Solidarity," this design system avoids heavy shadows or floating elements. Depth is communicated through **Tonal Layers** and **Low-Contrast Outlines**.

- **Level 0 (Background):** `Light Background (#F5F7FA)` used for the canvas.
- **Level 1 (Cards/Surface):** `White (#FFFFFF)` with a 1px `Border Gray (#DDE4EC)` stroke. This is the primary container for election info and forms.
- **Level 2 (Active/Hover):** A subtle 4px blur shadow with 5% opacity is only used on interactive cards (like selecting a candidate) to provide tactile feedback.
- **Interactive Elements:** Buttons use solid color fills. High-contrast outlines (2px) are used for focus states to ensure accessibility for keyboard users.

## Shapes

The shape language is **Soft (0.25rem)**. This slight rounding takes the "edge" off the institutional feel, making the platform accessible and inviting, while staying far enough away from "bubbly" or "playful" SaaS trends. 

- **Inputs and Buttons:** 4px (0.25rem) radius.
- **Election Cards:** 8px (0.5rem) radius.
- **Status Badges:** 2px radius (near-sharp) to maintain a serious, data-driven look.
- **Candidate Avatars:** These should be square or slightly rounded (8px), never circular, to mimic the look of official identification documents.

## Components

### Buttons
- **Primary:** Navy Blue background, White text. High-contrast, used for "Submit Vote" or "Confirm."
- **Secondary:** White background, Navy Blue border and text. Used for "Back" or "Cancel."
- **Destructive:** Dark Red background. Used for irreversible actions in admin panels.
- **Outline:** Transparent background, Border Gray stroke. Used for secondary administrative actions.

### Election Cards & Candidate Profiles
Cards must have a clear hierarchy: Candidate Name (Headline-MD), Party Affiliation (Body-SM), and a clear "Select" area. Use high-contrast selection states (Navy Blue border, 2px thickness) when a candidate is chosen.

### Status Badges
Small, rectangular badges with low-saturation backgrounds and high-saturation text.
- **Active:** Soft Green background / Success Green text.
- **Upcoming:** Soft Blue background / Navy Blue text.
- **Completed:** Gray background / Dark Gray text.

### Accessible Forms
Input fields must always have persistent labels. Error states must use `Election Red` for both the border and a descriptive error icon/text below the field. Bilingual support (English above, Nepali in a smaller, secondary weight below) is the default for all instructional labels.

### Data Tables
Dense, structured tables for administrative use. Use `Soft Blue Background` for the header row and 1px `Border Gray` for all cell dividers to ensure horizontal tracking of data.