export const meta = {
  name: 'knowledge-architect-agentic-workflow-audit',
  description: 'One stage of a pass of the agentic-workflow axis of a project audit, run only as that axis says',
  phases: [{ title: 'drafts' }, { title: 'clusters' }, { title: 'repairs' }],
}
// args: { stage, commit,
//   agents: [{ lens, group, scratch, kept }]     for drafts; group for L2 only, kept for a re-check only
//   findings, scratches: [dir]                    for clusters: the confirmed-findings file, one dir per agent
//   causesFile, causes: [{ id, scratch }]         for repairs }
const PATH = { type: 'object', properties: { path: { type: 'string' } }, required: ['path'] }
const CORPUS = 'The corpus is every text the harness delivers to a session as an instruction: the ' +
  'project root CLAUDE.md and the primer it imports, the installed skills and agents, and the ' +
  'project own skills and agents. Read every file of it whole. You edit nothing in the project: ' +
  'you write only in your scratch directory, and run only commands that read.'
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
    'Lens: ' + a.lens + (a.group ? ', group of activities: ' + a.group : ''),
    'Scratch directory: ' + a.scratch,
    a.kept ? 'Kept list, for this re-check: ' + a.kept : '',
    'Your copies of your definition, knowledge-architect-workflow-auditor.md in the project agent ' +
      'directory, and of the root CLAUDE.md may predate the edits of this session: read both from ' +
      'disk, and follow the disk where your copy differs.',
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
    'Propose one repair for the cause, never one per finding. By its kind: gather the ' +
      'instructions of the moment into one home, the section of the skill that owns the moment, ' +
      'and replace the others by a pointer to it; remove the structure; bring the restatement ' +
      'back to a pointer to its home; delete the narrow rule and leave the case to judgement. ' +
      'Add an instruction only where none of these closes the cause, and say why.',
    'An instruction is a sentence that tells an agent to do, or not to do, an act at a moment. ' +
      'A rewording that makes an existing instruction intent clearer adds none, whatever its length.',
    'Write proposal.md in your scratch directory, ' + c.scratch + ': each edit as its file, the ' +
      'text before and the text after; the findings each edit closes; every instruction added ' +
      'and removed, quoted, and their two counts; and any finding of the cause the repair does ' +
      'not close, with the reason. Return its path.',
  ]), { schema: PATH, phase: 'repairs', label: c.id })))
}
throw new Error('args.stage is drafts, clusters or repairs')
