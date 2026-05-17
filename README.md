# Devlog

Devlog is an ambient developer journal daemon that watches coding activity and turns it into useful daily progress notes.

It captures signals from development work such as commits, file changes, and shell activity, then stores them as structured project events. Those events can be summarized into standup updates with an LLM, helping developers remember what they worked on without manually writing logs throughout the day.

Devlog is meant to run quietly in the background and build a timeline of engineering activity across a project, including:

- Git commits and commit metadata
- Changed files and code churn
- Project and branch context
- Local development activity
- LLM-generated daily standup summaries

