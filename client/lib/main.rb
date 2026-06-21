require 'libui'

UI = LibUI

UI.init

main_window = UI.new_window('Miyuki X editor', 200, 700, 1)

# UI.new_text box('text_box')
# (ranndering) UI.new_viwer('viwer')
# btm(post)
button = UI.new_button('Button')

UI.button_on_clicked(button) do
  # do post request rust server
  UI.msg_box(main_window, 'Information', 'You clicked the button')
end

UI.window_set_child(main_window, button) # text box, vIwer box

UI.control_show(main_window) #sub widow - viwer

UI.main
UI.quit