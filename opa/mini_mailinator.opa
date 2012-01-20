/**
 * Mailinator clone in opa
 *
 * Mailinator is a free disposable e-mail address service. The idea is to let a
 * user create a new e-mail address on the fly, whenever needed, for instance
 * while filling a form on a web site.
 *
 * Mailinator will accept mail for any e-mail address within the mailinator.com
 * domain, and allows anyone to read it. There is no need to register for an
 * account or authenticate via a password.
 *
 * The flow looks like this:
 * 1. you use foobar@mailinator.com as your email address
 * 2. you can go look at what messages have arrived by going to
 *    foobar.mailinator.com
 *
 * I decided to write a quick mailinator clone for the following reasons:
 * - Mailinator mailboxes can be read by anyone. I therefore wanted to have an
 *   internal service.
 * - Mailinator filters spam & delays emails. This isn't ideal when mailinator
 *   becomes part of a testing framework.
 * - Mailinator does not store the messages for a very long period of time.
 *   I want to be able to control things.
 */

import stdlib.themes.bootstrap
import stdlib.web.mail
import stdlib.web.client

type message = {
  string from,
  string subject,
  string body,
  Date.date received
}

type mailbox_key_message = {
  string mailbox,
  int key,
  message message
}

database stringmap(intmap(message)) /messages

function SmtpServer.result process_email(string from, list(string) arr_to, string body) {
  List.map(
    function(to) {
      mailbox = List.head(String.explode("@", to))
      int id = Db.fresh_key(@/messages[mailbox])
      string subject = parse_subject(body)
      message m = {from:from, subject:subject, body:body, received:Date.now()}
      /messages[mailbox][id] <- m
      Network.broadcast(
        {mailbox:mailbox, key:id, message:m},
        Network.network(mailbox_key_message) (Network.cloud(mailbox))
      )
    },
    arr_to
  )
  {success}
}

function parse_subject(string body) {
  // body: headers followed by an empty line, followed by the email content
  // header: Key: value
  List.fold(
    function (string header, string r) {
      if (String.has_prefix("subject:", String.lowercase(header))) {
        String.substr(8, String.length(header)-8, header);
      } else {
        r;
      }
    },
    String.explode("\n", body),
    ""
  )
}

function xhtml render_subject(string subject) {
  if (subject == "") {
    <i>No Subject</i>
  } else {
    <>{subject}</>
  }
}

function delete(string mailbox, int item, string dom_id) {
  Dom.remove_content(Dom.select_id(dom_id))
  Db.remove(@/messages[mailbox][item])
  void
}

function xhtml render_message(string mailbox, int key, message m, string dom_id, string style) {
  <tr id={dom_id} style="{style}">
    <td>
      {m.from}
    </td>
    <td>
      <a href="/{mailbox}/{key}">{render_subject(m.subject)}</a>
    </td>
    <td>
      {Date.to_formatted_string(Date.default_printer, m.received)}
    </td>
    <td>
      <span onclick={function(_){delete(mailbox, key, dom_id)}} class="icon icon-color icon-close"/>
    </td>
  </tr>
}

function update_list(mailbox_key_message m) {
  dom_id = Dom.fresh_id()
  #rows =+ render_message(m.mailbox, m.key, m.message, dom_id, "display: none")
  Dom.transition(Dom.select_id(dom_id), Dom.Effect.with_duration({millisec:1000}, Dom.Effect.fade_in()))
  void
}

function get_list(string mailbox) {
  xhtml table = Map.fold(
    function(k, v, r) {
      <>
        {r}
        {render_message(mailbox, k, v, Dom.fresh_id(), "")}
      </>
    },
    /messages[mailbox],
    <></>
  )

  {
    title:"OpaMailinator | Inbox",
    body:
      <table class="zebra-striped"
        onready={function(_) {
          Network.add_callback(update_list,
          Network.network(mailbox_key_message) (Network.cloud(mailbox)))
        }}>
        <thead>
          <tr>
            <th>From</th>
            <th>Subject</th>
            <th>Received</th>
            <th></th>
          </tr></thead>
        <tbody id=#rows>{table}</tbody>
      </table>
  }
}

function get_message(string mailbox, int num) {
  message m = /messages[mailbox][num]

  {
    title:"OpaMailinator | {m.subject}",
    body:
      <pre>
        {m.body}
      </pre>
  }
}

function get_home() {
  {
    title: "OpaMailinator",
    body:
      <div class="well">
        <h3>Enter mailbox</h3>
        <input id=#mailbox type="text" onnewline={function(e){Client.goto("/{Dom.get_value(#mailbox)}")}}/>
        <input type="button" value="Go" onclick={function(e){Client.goto("/{Dom.get_value(#mailbox)}")}}/>
      </div>
  }
}

function resource display(content) {
  Resource.styled_page(
    content.title,
    [],
    <>
      <div class="container">
        <div class="content">
          <section>
            <div class="page-header"/>
            {content.body}
          </section>
        </div>
      </div>
    </>
  )
}

function resource start(Uri.relative uri) {
  match (uri) {
    case {path:{hd:mailbox, tl:tl} ...}:
      match (tl) {
        case {hd:num ...}:
          display(get_message(mailbox, Int.of_string(num)))
        case {nil}:
          display(get_list(mailbox))
      }
    case _ :
      display(get_home())
  }
}

SmtpServer.start(0.0.0.0, 25, {none}, process_email)
Server.start(Server.http, {dispatch: start})
