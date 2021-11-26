Consider a community of programmers (much like ourselves). They want to develop their programming skills together. Propose practice problems to each other, view each others solutions, comment on each others solutions, etc..

What if there were a website where these programmers could go to do those things. I'm not proposing reinventing leet code. I'm proposing more of a site that someone self hosts (on a raspberry pi for example), and privately invites the community members. Then members can propose problems, vote on the next problem for the week (or month, whatever). Then all members will have until the end time frame to propose their solutions. On the last day members could compare and comment on each others solutions. This would be a fun way for developers from that community to practice, learn from each other, and generally help members build rapport with each other through the exercise of discussing and solving the same problem together.

A couple User Stories:

- As a member of this site, I want to to discuss each weekly (or monthly) problem in an open forum. 
    - I want the discussion forum to enable markdown rendering (like github).
    - I want to be able to mark either my whole message, or part of my message as a spoiler.
    - I want to be able to edit my messages, while maintaining edit history (like github).
- As a member of this site, I want to to leave comments (markdown rendered) on specific sections (line x-z for example) of other member's proposed solutions. Similar to commenting on a PR in github.
- As a member of this site, I want to to view, and reply to comments left on my soltuion.
- As a member of this site, I want to to upload multiple iterations of my solution.
- As a member of this site, I want to propose new problems to the community, and be able to vote on the next problem.

- As an owner of this site, I want to be able to invite new members by sending them a link to sign up.
- As an owner of this site, I want to be able to host it anywhere. (Given that I've port forwarded, and bought a domain, etc..)
- As an owner of this site, I want to be able to manage members. This means making some members admins.
- As an owner of this site, I want to be able to give certain members, admin controls.

- As an admin of this site, I want to be able to moderate the discussion forum. This includes, marking messages as spoilers, and deleting messages. Any changes made will be recorded to an immutable audit log.
- As an admin of this site, I want to be able to manage and edit the proposed problems list. Any edits made will be recorded to an immutable audit log.
- As an admin of this site, I want all the priveldges of a Member.
