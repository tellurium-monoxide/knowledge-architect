export const meta = {
  name: 'knowledge-architect-agentic-workflow-audit',
  description: 'One stage of a pass of the agentic-workflow axis of a project audit, run only as that axis says',
  phases: [{ title: 'drafts' }, { title: 'clusters' }, { title: 'repairs' }],
}
// args: { stage, commit,
//   agents: [{ lens, group, scratch, worktree, kept }]   for drafts; group for L2 only, kept for a re-check only
//   findings, scratches: [dir]                    for clusters: the confirmed-findings file, one dir per agent
//   causesFile, causes: [{ id, scratch }]         for repairs }
const PATH = { type: 'object', properties: { path: { type: 'string' } }, required: ['path'] }
const CORPUS = 'The corpus is every text the harness delivers to a session as an instruction, ' +
  'installed or the project own: the project root CLAUDE.md and the primer it imports, and every ' +
  'other such text in the project agent configuration directory. Read every file of it whole. You ' +
  'edit nothing in the project: you write only in your scratch directory, and run only commands ' +
  'that read.'
const KINDS = 'A cause is one of four kinds. Two homes for one moment: instructions about one ' +
  'moment or one kind of act stated in more than one text. A structure the work did not need: a ' +
  'fixed sequence, count or mapping that another instruction needs to vary. A restatement that ' +
  'drifted from its home. A rule written for one interaction: a narrow rule where judgement ' +
  'already decides.'
const lines = (xs) => xs.filter(Boolean).join('\n')
if (args.stage === 'drafts') {
  phase('drafts')
  return await parallel(args.agents.map((a) => () => agent(lines([
    'Commit audited: ' + args.commit,
    'Worktree, detached at that commit: ' + a.worktree,
    'Lens: ' + a.lens + (a.group ? ', group of activities: ' + a.group : ''),
    'Scratch directory: ' + a.scratch,
    a.kept ? 'Kept list, for this re-check: ' + a.kept : '',
    'Your copies of your definition, knowledge-architect-workflow-auditor.md in the project agent ' +
      'directory, and of the root CLAUDE.md may predate the edits of this session: read both in ' +
      'your worktree, and follow the worktree where your copy differs.',
  ]), { agentType: 'knowledge-architect-workflow-auditor', phase: 'drafts', label: a.lens })))
}
if (args.stage === 'clusters') {
  phase('clusters')
  return await parallel(args.scratches.map((dir, i) => () => agent(lines([
    'You group the confirmed findings of one pass of the agentic-workflow axis of a project ' +
      'audit by the cause that produced them, at commit ' + args.commit + '.',
    CORPUS,
    'Read the confirmed-findings file ' + args.findings + ', every draft it names, and the corpus.',
    KINDS + ' A finding that fits none stands alone.',
    'For each cause write: an id, its kind, its mechanism in one sentence, the ids of its ' +
      'findings, and every text, with its path and section, that states an instruction about ' +
      'its moment or its act.',
    'Write the grouping to grouping.md in your scratch directory, ' + dir + ', and return its path.',
  ]), { schema: PATH, phase: 'clusters', label: 'cluster ' + (i + 1) })))
}
if (args.stage === 'repairs') {
  phase('repairs')
  return await parallel(args.causes.map((c) => () => agent(lines([
    'You propose the repair of one cause of findings of the agentic-workflow axis of a project ' +
      'audit, at commit ' + args.commit + '.',
    CORPUS,
    'Read the cause ' + c.id + ' in the causes file ' + args.causesFile + ', every draft its ' +
      'findings name, and the corpus. ' + KINDS,
    'Propose the repair as point 2, "The repair proposals", of "The repair by cause" in the ' +
      'section agentic-workflow-axis of the skill knowledge-architect-project-audit says: read ' +
      'that point whole. Where you add an instruction, say why none of its forms closes the cause.',
    'Write proposal.md in your scratch directory, ' + c.scratch + ': each edit as its file, the ' +
      'text before and the text after; the findings each edit closes; every instruction added ' +
      'and removed, quoted, and their two counts; and any finding of the cause the repair does ' +
      'not close, with the reason. Return its path.',
  ]), { schema: PATH, phase: 'repairs', label: c.id })))
}
throw new Error('args.stage is drafts, clusters or repairs')
