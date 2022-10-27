class TestCase
  def initialize(name)
    @name = name
  end

  def run
    send(@name)
  end
end
