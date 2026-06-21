require 'libui'

UI = LibUI
UI.init

begin
  main_window = UI.new_window('Miyuki X editor', 200, 700, 1)
rescue 
  catch(:init) do
    throw :init if !main_window? 
end
# UI.new_text box('text_box')
# (ranndering) UI.new_viwer('viwer')
button = UI.new_button('Button') #post

UI.button_on_clicked(button) do
  # do post request rust server
  UI.msg_box(main_window, 'Information', 'You clicked the button')
end

# options
UI.window_set_child(main_window, button) # text box, vIwer box
UI.control_show(main_window) #sub widow - viwer

# end
UI.main
UI.quit