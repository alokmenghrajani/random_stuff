/**
 * Mailinator clone in opa
 */

import stdlib.themes.bootstrap
import stdlib.web.mail

type message = {
  string from,
  string subject,
  string content,
}

database stringmap(intmap(message)) /messages

function SmtpServer.result process_email(string from, list(string) arr_to, string content) {
  List.map(
    function(to) {
      mailbox = List.head(String.explode("@", to))
      int id = Db.fresh_key(@/messages[mailbox])
      string subject = parse_subject(content)
      /messages[mailbox][id] <- {~from, ~subject, ~content}
    },
    arr_to
  )
  {success}
}

function parse_subject(string content) {
  // content: {headers} followed by an empty line, followed by {body}
  // header:  "key: value"
  List.fold(
    function (string header, string r) {
      if (String.has_prefix("subject:", String.lowercase(header))) {
        String.substr(8, String.length(header)-8, header);
      } else {
        r;
      }
    },
    String.explode("\n", content),
    ""
  )
}

function get_list(string mailbox) {
  xhtml table = Map.fold(
    function(k, v, r) {
      <>
        {r}
        <tr>
          <td>{v.from}</td>
          <td><a href="/{mailbox}/{k}">{v.subject}</a></td>
        </tr>
      </>
    },
    /messages[mailbox],
    <></>
  )

  <table class="zebra-striped">
    <thead><tr><th>From</th><th>Subject</th></tr></thead>
    <tbody>{table}</tbody>
  </table>
}

function get_message(string mailbox, int num) {
  message m = /messages[mailbox][num]

  <pre>
    {m.content}
  </pre>
}

function resource display(xhtml content) {
  Resource.styled_page("Opa Mailinator", [],
    <div class="container"><div class="content"><section>
      <div class="page-header"/>
      {content}
    </section></div></div>
  )
}

function resource start(Uri.relative uri) {
  match (uri) {
    case {path:{hd:mailbox, tl:[]} ...}:
      display(get_list(mailbox))
    case {path:{hd:mailbox, tl:[num]} ...}:
      display(get_message(mailbox, Int.of_string(num)))
  }
}

SmtpServer.start(0.0.0.0, 25, {none}, process_email)
Server.start(Server.http, {dispatch: start})
